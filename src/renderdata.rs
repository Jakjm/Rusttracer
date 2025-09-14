use std::fmt;
use std::io;
use std::io::{Error, ErrorKind};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;
use crate::matrix::Vector4;
use crate::shape::{LightingProps,Shape};
use crate::elements::{Cube, Sphere, Tetrahedron, Light};
use image::codecs::pnm::{PnmEncoder, PnmSubtype, SampleEncoding};
use image::codecs::png::PngEncoder;
use image::{ImageEncoder, Rgb};
use crossbeam::scope;
pub struct RenderData{
    near: f64,
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    width: usize, 
    height: usize,
    spheres: Vec<Sphere>,
    tetras: Vec<Tetrahedron>,
    cubes: Vec<Cube>,
    lights: Vec<Light>,
    back_color: Vector4,
    amb_color: Vector4,
    output_ppm_file: String,
    output_png_file: String,
}
const NUM_BOUNCES : i32 = 3;
impl RenderData{
    pub fn save_image(&self, raw_pixels: Vec<u8>) -> std::io::Result<()>{
        let rgb_image = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(self.width as u32, self.height as u32, raw_pixels).unwrap();

        let ppm_path = Path::new(&self.output_ppm_file);
        let ppm_file = File::create(ppm_path)?;
        let ppm_writer = BufWriter::new(ppm_file);
        let pnm_encoder = PnmEncoder::new(ppm_writer).with_subtype(PnmSubtype::Pixmap(SampleEncoding::Binary));
        if pnm_encoder.write_image(&rgb_image, self.width as u32, self.height as u32, image::ExtendedColorType::Rgb8).is_err() {
            return Err(Error::new(ErrorKind::Other, format!("Error when writing png file.")));
        }
        //Originally used the following according to this specification:
        //https://paulbourke.net/dataformats/ppm/
        //If ppm_writer is mutable, can use the following:
        // let opening_string = format!("P6\n{} {}\n{}\n", self.width, self.height, 255);
        // ppm_writer.write_all(opening_string.as_bytes())?; 
        // ppm_writer.write_all(&raw_pixels)?;

        let png_path = Path::new(&self.output_png_file);
        let png_file = File::create(png_path)?;
        let png_writer = BufWriter::new(png_file);
        let png_encoder = PngEncoder::new(png_writer);
        if png_encoder.write_image(&rgb_image, self.width as u32, self.height as u32, image::ExtendedColorType::Rgb8).is_err() {
            return Err(Error::new(ErrorKind::Other, format!("Error when writing png file.")));
        }

        Ok(())
    }

    pub fn check_collisions(&self, origin: &Vector4, ray: &Vector4, min: f64, max: f64) -> Option<(&dyn Shape, Vector4,Vector4)> {
        let mut lowest = max;
        let mut col_data = None;
        for sphere in self.spheres.iter(){
            if let Some((t, col_pt, normal)) = sphere.check_collision(origin, ray, min, lowest){
                col_data = Some((sphere as &dyn Shape, col_pt, normal));
                lowest = t;
            }
        }
        for cube in self.cubes.iter(){
            if let Some((t, col_pt, normal)) = cube.check_collision(origin, ray, min, lowest){
                col_data = Some((cube as &dyn Shape, col_pt, normal));
                lowest = t;
            }
        }
        for tetra in self.tetras.iter(){
            if let Some((t, col_pt, normal)) = tetra.check_collision(origin, ray, min, lowest){
                col_data = Some((tetra as &dyn Shape, col_pt, normal));
                lowest = t;
            }
        }

        return match col_data {
            None => None,
            Some((shape, col_pt, mut normal)) => {
                normal.normalize();
                Some((shape, col_pt, normal))
            }
        };
    }

    pub fn compute_light_color(&self, col_pt: &Vector4, ray: &Vector4, normal: &Vector4, shape: &dyn Shape) -> Vector4{
        let LightingProps {color: shape_color, amb, diff, spec, refl: _, bright} = shape.lighting_props();
        let mut light_color = self.amb_color.clone();
        light_color *= *amb;
        light_color *= shape_color;

        for light in self.lights.iter(){
            let mut shadow_ray = light.pos.clone();
            shadow_ray -= col_pt;

            let dot = shadow_ray.dot(normal);
            if dot < 0.0 || self.check_collisions(col_pt, &shadow_ray, 0.000000001, 1.0).is_some()  {
                continue;
            }
            
            let mut diff_color = light.intensity.clone();
            diff_color *= (dot * diff) / shadow_ray.len();
            diff_color *= shape_color;
            light_color += &diff_color; //Computed and added diffuse light

            //Calculate the amount that the ray bounces off of surface.
            let mut bounce = normal.clone();
            bounce *= -2.0 * dot;

            let mut ref_ray = shadow_ray; //Calculating reflection of shadow ray off shape
            ref_ray *= -1.0;
            ref_ray -= &bounce;
            
            let mut shininess = -ray.dot(&ref_ray);
            if shininess > 0.0 && *spec > 0.0{
                shininess /= ray.len();
                shininess /= ref_ray.len();
                shininess = shininess.powf(*bright);

                let mut spec_color = light.intensity.clone();
                spec_color *= shininess * spec;

                light_color += &spec_color;
            }
        }
        return light_color;
    }
    
    pub fn traceray(&self, origin :&Vector4, ray: &Vector4, min_t:f64, bounce_ct : i32) -> Vector4{
        let color = match self.check_collisions(&origin, &ray, min_t, std::f64::INFINITY) {
            None => match bounce_ct < NUM_BOUNCES { //If this is a bounced ray, return black if there were no collisions.
                true => Vector4::vec(0.0, 0.0, 0.0),
                false => self.back_color.clone(),
            },
            Some((shape, col_pt, normal)) => {
                let mut color = self.compute_light_color(&col_pt, &ray, &normal, shape);
                if bounce_ct > 0 {
                    let dot = 2.0 * ray.dot(&normal);
                    let mut bounce = normal;
                    bounce *= dot;

                    let mut refl_ray = ray.clone();
                    refl_ray -= &bounce;

                    let mut ref_color = self.traceray(&col_pt, &refl_ray, 0.0000001, bounce_ct - 1);
                    ref_color *= shape.lighting_props().refl;
                    color += &ref_color;
                }
                color
            },
        };
        return color;
    }
    pub fn render_slice(&self, slice: &mut [u8], start_y: usize, end_y: usize, extra_points : u32){
        let eye = Vector4::point(0.0,0.0,0.0);
        let pixel_width = (self.right - self.left) / self.width as f64;
        let pixel_height = (self.top - self.bottom) / self.height as f64;
        let mut ray = Vector4::vec(0.0, 0.0 ,-self.near);
        for px_y in start_y..end_y{
            for px_x in 0..self.width{
                let mut num_samples : f64 = 1.0;
                let mut average_color = Vector4::vec(0.0, 0.0, 0.0);

                let _print_check = false;
                let pixel_center_x = self.left + pixel_width * (px_x as f64 + 0.5);
                let pixel_center_y = self.top - pixel_height * (px_y as f64 + 0.5);
                ray.arr[0] = pixel_center_x;
                ray.arr[1] = pixel_center_y;
                let color = self.traceray(&eye, &ray, 1.0000001, NUM_BOUNCES);
                average_color += &color;

                for i in 0..extra_points{
                    let angle = 2.0 * std::f64::consts::PI * (i as f64 / extra_points as f64) + 0.25 * std::f64::consts::PI;

                    let variance_x = 0.65 * angle.cos();
                    let variance_y = 0.65 * angle.sin();
                    ray.arr[0] =  pixel_center_x + pixel_width * variance_x; 
                    ray.arr[1] =  pixel_center_y + pixel_height * variance_y;
                    let mut color = self.traceray(&eye, &ray, 1.0000001, NUM_BOUNCES);
                    color *= 0.7;
                    average_color += &color;
                    num_samples += 0.7;

                }

                average_color /= num_samples;
                let index: usize = 3 * ((px_y - start_y) * self.width + px_x) as usize;
                let (red, green, blue) = average_color.to_rgb();
                slice[index] = red;
                slice[index + 1] = green;
                slice[index + 2] = blue;
            }
        }
    }
    fn split_into_equal_chunks<T>(mut vec: &mut [T], n: usize, ensure_multiple : usize) -> (usize, Vec<&mut [T]>){
        let len : usize = vec.len();
        let chunk_size = ensure_multiple * (len / (ensure_multiple * n));

        let mut result = Vec::<&mut [T]>::with_capacity(n);
        for _i in 0..(n - 1){
            let (left, right) = vec.split_at_mut(chunk_size);
            let _len = left.len();
            result.push(left);
            vec = right;
        }
        let _len = vec.len();
        result.push(vec);
        return (chunk_size, result);
    }
    pub fn render(&self, extra_points: u32, thread_count: usize) -> Vec<u8>{
        let capacity: usize = (3 * self.width * self.height) as usize;
        let mut array = Vec::<u8>::with_capacity(capacity);
        unsafe{
            array.set_len(capacity);
        }
        let (chunk_size, mut chunks) = Self::split_into_equal_chunks(&mut array, thread_count, 3 * self.width);
        let frac = chunk_size / (3 * self.width);
        let mut start_y = 0;
        let mut end_y = frac;
        let _ = scope(|s| {
            let mut handles = Vec::with_capacity(thread_count - 1);
            let mut iter = chunks.iter_mut();
            for _t in 0..(thread_count - 1) {
                let slice = iter.next().unwrap();
                let handle = s.spawn(move |_| {
                    self.render_slice(slice, start_y, end_y, extra_points);
                });
                handles.push(handle);
                start_y = end_y;
                end_y += frac;
            }
            let slice = iter.next().unwrap();
            self.render_slice(slice, start_y, self.height, extra_points);
            for handle in handles{
                let _result = handle.join();
            }
        });
        return array;
    }
    //TODO replace with detailed error messages....
    pub fn read_resolution(tokens: &Vec<&str>) -> Option<(usize, usize)>{
        if tokens.len() != 3 {
            return None;
        }

        let width_res = tokens[1].to_string().trim().parse::<usize>();
        let height_res = tokens[2].to_string().trim().parse::<usize>();
        if width_res.is_err() || height_res.is_err(){
            return None;
        }

        let width = width_res.unwrap();
        let height = height_res.unwrap();
        if width == 0 || height == 0 {
            return None;
        }
        return Some((width, height));
    }
    //TODO replace with detailed error messages...
    pub fn read_scene_param(tokens: &Vec<&str>, positive: bool) -> Option<f64>{
        if tokens.len() != 2 {
            return None;
        }

        let result = tokens[1].to_string().trim().parse::<f64>();
        if result.is_err(){
            return None;
        }

        let param_val = result.unwrap();
        if (positive && param_val <= 0.0) || (!positive && param_val >= 0.0) {
            return None;
        }
        return Some(param_val);
    }

    pub fn read_from_file(filename: &String) -> Result<Self, io::Error>{
        let path = Path::new(&filename);
        let file_result = File::open(&path);
        if file_result.is_err() {
            return Err(Error::new(ErrorKind::Other, format!("Couldn't open {filename} for reading!"))); 
        }
        let mut reader = BufReader::new(file_result.unwrap());
        let lines = (&mut reader).lines();
        
        let mut spheres = Vec::<Sphere>::new();
        let mut cubes = Vec::<Cube>::new();
        let mut tetras = Vec::<Tetrahedron>::new();
        let mut lights = Vec::<Light>::new();

        let (mut near, mut left, mut right, mut bottom, mut top) = (None, None, None, None, None);
        let mut resolution : Option<(usize, usize)> = None; 
        let mut amb_color: Option<Vector4> = None;
        let mut back_color: Option<Vector4> = None;
        let mut output_file: Option<String> = None;
        for line in lines.map_while(Result::ok){
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() <= 0 {
                continue;
            }
            let first_token = tokens[0];
            match first_token {
                "SPHERE" => {
                    match Sphere::read_from_tokens(&tokens) {
                        None => return Err(Error::new(ErrorKind::Other, format!("Could not read sphere from {line}."))),
                        Some(sphere) => spheres.push(sphere),
                    }
                },
                "CUBE" => {
                    match Cube::read_from_tokens(&tokens) {
                        None => return Err(Error::new(ErrorKind::Other, format!("Could not read sphere from {line}."))),
                        Some(cube) => cubes.push(cube),
                    }
                },
                "TETRA" => {
                    match Tetrahedron::read_from_tokens(&tokens) {
                        None => return Err(Error::new(ErrorKind::Other, format!("Could not read tetrahedron from {line}."))),
                        Some(tetra) => tetras.push(tetra),
                    }
                },
                "LIGHT" => {
                    match Light::read_from_tokens(&tokens){
                        None => return Err(Error::new(ErrorKind::Other, format!("Could not read light from {line}."))),
                        Some(light) => lights.push(light),
                    }
                },
                "RES" => {
                    if resolution.is_some() {
                        return Err(Error::new(ErrorKind::Other, "Only one line for the resolution is permitted!"));
                    }
                    match Self::read_resolution(&tokens){
                        None => return Err(Error::new(ErrorKind::Other, format!("Could not read resolution from {line}."))),
                        Some(res) => resolution = Some(res),
                    }
                },
                "NEAR" | "LEFT" | "RIGHT" | "BOTTOM" | "TOP"  => {
                    let (value_reference, positive) : (&mut Option<f64>, bool) = match first_token {
                        "NEAR"  => (&mut near, true),
                        "LEFT"  => (&mut left, false),
                        "RIGHT" => (&mut right, true),
                        "BOTTOM" => (&mut bottom, false),
                        "TOP" => (&mut top, true),
                        &_ => unreachable!(),
                    };

                    if (*value_reference).is_some(){
                        return Err(Error::new(ErrorKind::Other, format!("Only one line for {first_token} is permitted!")));
                    }
                    match Self::read_scene_param(&tokens, positive){
                        None => return Err(Error::new(ErrorKind::Other, format!("Could not read {first_token} from {line}"))),
                        Some(value) => *value_reference = Some(value),
                    }
                }
                "BACK" | "AMBIENT" => {
                    let color_reference : &mut Option<Vector4> = match first_token{
                        "BACK" => &mut back_color,
                        "AMBIENT" => &mut amb_color,
                        &_ => unreachable!(),
                    };

                    if (*color_reference).is_some(){
                        return Err(Error::new(ErrorKind::Other, format!("Only one line for {first_token} is permitted!")));
                    }
                    match Vector4::vec_from_str_tokens(&tokens){
                        None => return Err(Error::new(ErrorKind::Other, format!("Could not read {first_token} colour from {line}."))),
                        Some(color) => *color_reference = Some(color),
                    }
                },
                "OUTPUT" => {
                    if output_file.is_some(){
                        return Err(Error::new(ErrorKind::Other, "Only one line for output file permitted!"));
                    }
                    match tokens.len(){
                        2 => output_file = Some(tokens[1].to_string().trim().to_string()),
                        _ => return Err(Error::new(ErrorKind::Other, format!("Could not read output file from {line}."))),
                    }
                },
                &_ => continue,
            }
        }
        
        let (near, left, right, bottom, top) = (near.unwrap_or(1.0), left.unwrap_or(-1.0), right.unwrap_or(1.0), bottom.unwrap_or(-1.0), top.unwrap_or(1.0));
        let (width, height) : (usize, usize) = resolution.unwrap_or((800, 600));
        let (back_color, amb_color) : (Vector4, Vector4) = (back_color.unwrap_or(Vector4::vec(1.0, 1.0, 1.0)), amb_color.unwrap_or(Vector4::vec(1.0, 1.0, 1.0)));
        let output_ppm_file = output_file.unwrap_or("output.ppm".to_string());
        let output_png_file = output_ppm_file.trim_end_matches(".ppm").to_string() + ".png";
        
        let result = Self{near, left, right, bottom, top, width, height, 
            spheres, tetras, cubes, lights, back_color, amb_color, output_ppm_file, output_png_file};
        return Ok(result);
    }
}

impl fmt::Display for RenderData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scene Resolution: {}x{} pixels\n", self.width, self.height)?;
        write!(f, "Near plane: {}, Horizontal range: {{{},{}}} Vertical range: {{{},{}}}\n", self.near, self.left, self.right, self.bottom, self.top)?;
        write!(f, "Back colour: {}, Ambient colour:{}\n", self.back_color, self.amb_color)?;
        write!(f, "\nShapes:\n")?;
        for tetra in self.tetras.iter(){
            write!(f,"\t-{tetra}\n")?;
        }
        for cube in self.cubes.iter() {
            write!(f, "\t-{cube}\n")?;
        }
        for sphere in self.spheres.iter() {
            write!(f, "\t-{sphere}\n")?;
        }
        write!(f, "\nLights:\n")?;
        for light in self.lights.iter(){
            write!(f, "\t-{light}\n")?;
        }
        write!(f, "\nOutput filenames: {} {}", self.output_ppm_file, self.output_png_file)?;
        return Ok(());
    }
}
