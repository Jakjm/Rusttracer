use std::fmt;
use std::io;
use std::io::{Error, ErrorKind};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
//use std::io::{Write}
use std::path::Path;
use crate::elements::Shape;
use crate::matrix::Vector4;
use crate::elements::Sphere;
use crate::elements::Light;
use image::codecs::pnm::{PnmEncoder, PnmSubtype, SampleEncoding};
use image::codecs::png::PngEncoder;
use image::{ImageEncoder, Rgb};
pub struct RenderData{
    near: f64,
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    width: u32, 
    height: u32,
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
    back_color: Vector4,
    amb_color: Vector4,
    array: Vec<Vector4>,
    output_ppm_file: String,
    output_png_file: String,
}

impl RenderData{
    pub fn save_image(&self) -> std::io::Result<()>{
        let capacity = (3 * self.width * self.height) as usize;
        let mut raw_pixels = Vec::<u8>::with_capacity(capacity);
        for pixel in self.array.iter(){
            let (red, green, blue) = pixel.to_rgb();
            raw_pixels.push(red);
            raw_pixels.push(green);
            raw_pixels.push(blue);
        }

        let rgb_image = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(self.width, self.height, raw_pixels).unwrap();

        let ppm_path = Path::new(&self.output_ppm_file);
        let ppm_file = File::create(ppm_path)?;
        let ppm_writer = BufWriter::new(ppm_file);
        let pnm_encoder = PnmEncoder::new(ppm_writer).with_subtype(PnmSubtype::Pixmap(SampleEncoding::Binary));
        if pnm_encoder.write_image(&rgb_image, self.width, self.height, image::ExtendedColorType::Rgb8).is_err() {
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
        if png_encoder.write_image(&rgb_image, self.width, self.height, image::ExtendedColorType::Rgb8).is_err() {
            return Err(Error::new(ErrorKind::Other, format!("Error when writing png file.")));
        }

        Ok(())
    }

    pub fn check_collisions(&self, origin: &Vector4, ray: &Vector4, min: f64, max: f64) -> Option<(&dyn Shape,Vector4,Vector4)> {
        let lowest = std::f64::INFINITY;
        let mut col_data = None;
        for sphere in self.spheres.iter(){
            if let Some((t, col_pt, normal)) = sphere.check_collision(origin, ray, min, max){
                if t < lowest{
                    col_data = Some((sphere as &dyn Shape, col_pt, normal));
                }
            }
        }
        return col_data;
    }

    pub fn compute_light_color(&self, col_pt: &Vector4, ray: &Vector4, normal: &Vector4, shape: &dyn Shape) -> Vector4{
        let (shape_color, amb, diff, spec, bright) = shape.lighting_params();

        let mut color = self.amb_color.clone();
        color *= amb;
        color *= shape_color;

        for light in self.lights.iter(){
            let mut shadow_ray = light.pos.clone();
            shadow_ray -= col_pt;

            let mut dot = shadow_ray.dot(normal);
            if dot < 0.0 || self.check_collisions(col_pt, &shadow_ray, 0.000000001, 1.0).is_some()  {
                continue;
            }

            let normal_len_sq = normal.dot(normal);
            let normal_len = normal_len_sq.sqrt();
            let shadow_ray_len = shadow_ray.dot(&shadow_ray).sqrt();

            dot /= shadow_ray_len;
            dot /= normal_len;

            let mut diff_color = light.intensity.clone();
            diff_color *= dot;
            diff_color *= diff;
            diff_color *= shape_color;
            color += &diff_color; //Computed and added diffuse light

            shadow_ray *= -1.0;
            let dot = 2.0 * shadow_ray.dot(normal) / normal_len_sq;
            let mut bounce = normal.clone();
            bounce *= dot;

            let mut ref_ray = shadow_ray.clone(); //Calculating reflection of shadow ray off shape
            ref_ray -= &bounce;
            
            let mut shininess = -ray.dot(&ref_ray);
            if shininess > 0.0 && spec > 0.0{
                shininess /= ray.dot(ray).sqrt();
                shininess /= ref_ray.dot(&ref_ray).sqrt();
                shininess = shininess.powf(bright);

                let mut spec_color = light.intensity.clone();
                spec_color *= shininess;
                spec_color *= spec;
                color += &spec_color;
            }
        }
        return color;
    }

    pub fn traceray(&self, origin :&Vector4, ray: &Vector4, min_t:f64, bounce_ct : i32) -> Vector4{
        let color = match self.check_collisions(&origin, &ray, min_t, std::f64::INFINITY) {
            None => self.back_color.clone(),
            Some((shape, col_pt, normal)) => {
                let mut color = self.compute_light_color(&col_pt, &ray, &normal, shape);
                if bounce_ct > 0 {
                    let normal_len_sq = normal.dot(&normal);
                    let dot = 2.0 * ray.dot(&normal) / normal_len_sq;
                    let mut bounce = normal.clone();
                    bounce *= dot;

                    let mut refl_ray = ray.clone();
                    refl_ray -= &bounce;

                    let mut ref_color = self.traceray(&col_pt, &refl_ray, 0.0000001, bounce_ct - 1);
                    ref_color *= shape.refl();
                    color += &ref_color;
                }
                color
            },
        };
        return color;
    }
    pub fn render(&mut self, extra_points: u32){
        let eye = Vector4::point(0.0,0.0,0.0);
        for px_y in 0..self.height{
            for px_x in 0..self.width{
                let mut num_samples : f64 = 1.0;
                let mut average_color = Vector4::vec(0.0, 0.0, 0.0);

                let x : f64 = self.left + (self.right - self.left) * ((px_x as f64 + 0.5) / self.width as f64);
                let y : f64 = self.top - (self.top - self.bottom) * ((px_y as f64 + 0.5) / self.height as f64);
                let ray = Vector4::vec(x,y, -self.near); //Ray directly in the center of pixel at (x,y).
                let color = self.traceray(&eye, &ray, 1.0000001, 3);
                average_color += &color;

                for i in 0..extra_points{
                    let angle = 2.0 * std::f64::consts::PI * (i as f64 / extra_points as f64) + 0.25 * std::f64::consts::PI;
                    let variance_x = 0.5 + 0.65 * angle.cos();
                    let variance_y = 0.5 + 0.65 * angle.sin();
                    let x : f64 = self.left + (self.right - self.left) * ((px_x as f64 + variance_x) / self.width as f64);
                    let y : f64 = self.top - (self.top - self.bottom) * ((px_y as f64 + variance_y) / self.height as f64);

                    let ray = Vector4::vec(x, y, -self.near);
                    let mut color = self.traceray(&eye, &ray, 1.0000001, 3);
                    color *= 0.4;
                    average_color += &color;
                    num_samples += 0.4;

                }

                average_color /= num_samples;
                self.array[(px_y * self.height + px_x) as usize] = average_color;
            }
        }
    }
    //TODO replace with detailed error messages....
    pub fn read_resolution(tokens: &Vec<&str>) -> Option<(u32, u32)>{
        if tokens.len() != 3 {
            return None;
        }

        let width_res = tokens[1].to_string().trim().parse::<u32>();
        let height_res = tokens[2].to_string().trim().parse::<u32>();
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
        let mut lights = Vec::<Light>::new();

        let (mut near, mut left, mut right, mut bottom, mut top) = (None, None, None, None, None);
        let mut resolution : Option<(u32, u32)> = None; 
        let mut amb_color: Option<Vector4> = None;
        let mut back_color: Option<Vector4> = None;
        let mut output_file: Option<String> = None;
        for line in lines.map_while(Result::ok){
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let first_token = tokens[0];
            match first_token {
                "SPHERE" => {
                    match Sphere::read_from_tokens(&tokens) {
                        None => return Err(Error::new(ErrorKind::Other, format!("Could not read sphere from {line}."))),
                        Some(sphere) => spheres.push(sphere),
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
                &_ => return Err(Error::new(ErrorKind::Other, "Unrecognized token in file!")),
            }
        }
        
        let (near, left, right, bottom, top) = (near.unwrap_or(1.0), left.unwrap_or(-1.0), right.unwrap_or(1.0), bottom.unwrap_or(-1.0), top.unwrap_or(1.0));
        let (width, height) : (u32, u32) = resolution.unwrap_or((800, 600));
        let (back_color, amb_color) : (Vector4, Vector4) = (back_color.unwrap_or(Vector4::vec(1.0, 1.0, 1.0)), amb_color.unwrap_or(Vector4::vec(1.0, 1.0, 1.0)));
        let output_ppm_file = output_file.unwrap_or("output.ppm".to_string());
        let output_png_file = output_ppm_file.trim_end_matches(".ppm").to_string() + ".png";
        
        let capacity = (width * height) as usize;
        let array : Vec<Vector4> = vec![back_color.clone(); capacity];
        let result = Self{near, left, right, bottom, top, width, height, 
            spheres, lights, back_color, amb_color, array, output_ppm_file, output_png_file};
        return Ok(result);
    }
}

impl fmt::Display for RenderData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scene Resolution: {}x{} pixels\n", self.width, self.height)?;
        write!(f, "Near plane: {}, Horizontal range: {{{},{}}} Vertical range: {{{},{}}}\n", self.near, self.left, self.right, self.bottom, self.top)?;
        write!(f, "Back colour: {}, Ambient colour:{}\n", self.back_color, self.amb_color)?;
        write!(f, "\nShapes:\n")?;
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