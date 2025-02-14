use std::env;
use std::fmt;
use std::io;
use std::io::{Error, ErrorKind};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::matrix::Vector4;
use crate::matrix::Matrix4;
use crate::elements::Sphere;
use crate::elements::Light;
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
    output_file: String,
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

        let path = Path::new(&self.output_file);
        let mut file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        let opening_string = format!("P6\n{} {}\n{}\n", self.width, self.height, 255);
        writer.write_all(opening_string.as_bytes())?; 
        writer.write_all(&raw_pixels);

        Ok(())
    }

    pub fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64) -> Option<(&Sphere,f64,Vector4,Vector4)> {
        let mut lowest = std::f64::INFINITY;
        let mut col_data : Option<(&Sphere,f64,Vector4,Vector4)> = None;
        for sphere in self.spheres.iter(){
            let origin_prime = &sphere.inv_matrix * origin;
            let ray_prime = &sphere.inv_matrix * ray;
            
            let a = ray_prime.dot(&ray_prime);
            let b = origin_prime.dot(&ray_prime);
            let c = origin_prime.dot(&origin_prime) - 1.0;

            let det = b * b - a * c;
            if det >= 0.0{
                let sqrt_det = det.sqrt();
                let mut t = (-b - sqrt_det) / a;
                //Ray missed the front of the sphere.
                //Try for a collision for the back of the sphere.
                if t < min { 
                    t = (-b + sqrt_det) / a;
                }
                if t > min && t < lowest{
                    //Calculate the collision point on sphere...
                    let mut col_pt = ray.clone();
                    col_pt *= t;
                    col_pt += &origin;
                    col_pt.force_point();
        
                    //Calculate the normal of collision...
                    let mut col_pt_prime = ray_prime.clone();
                    col_pt_prime *= t;
                    col_pt_prime += &origin_prime;
        
                    if ray_prime.dot(&ray_prime) > origin_prime.dot(&origin_prime) {
                        col_pt_prime *= -1.0;
                    }
                    col_pt_prime.force_vec();
                    let mut normal =  &sphere.inv_transp * &col_pt_prime;
                    normal.force_vec();

                    col_data = Some((sphere,t, col_pt, normal));
                    lowest = t;
                }
            }
        }
        return col_data;
    }

    pub fn computeLightColor(&self, col_pt: &Vector4, ray: &Vector4, normal: &Vector4, sphere: &Sphere) -> Vector4{
        let mut color = Vector4::zero();

        for light in self.lights.iter(){
            let mut shadow_ray = light.pos.clone();
            shadow_ray -= col_pt;

            let mut dot = shadow_ray.dot(normal);
            if dot < 0.0 {
                continue;
            }
            let mut result = self.check_collision(col_pt, &shadow_ray, 0.000000001);
            
            let mut light_blocked: bool = false; 
            if let Some((dummy,t, col_pt, normal)) = result{
                if t < 1.0{
                    light_blocked = true;
                }
            }
            if !light_blocked {
                let normal_len_sq = normal.dot(normal);
                let normal_len = normal_len_sq.sqrt();
                let shadow_ray_len = shadow_ray.dot(&shadow_ray).sqrt();

                dot /= shadow_ray_len;
                dot /= normal_len;

                let mut diff = light.intensity.clone();
                diff *= dot;
                diff *= sphere.diff;
                diff *= &sphere.color;

                color += &diff; //Computed and added diffuse light

                shadow_ray *= -1.0;
                let dot = 2.0 * shadow_ray.dot(normal) / normal_len_sq;
                let mut bounce = normal.clone();
                bounce *= dot;

                let mut ref_ray = shadow_ray.clone(); //Calculating reflection of shadow ray about the normal of sphere.
                ref_ray -= &bounce;
                
                let mut shininess = -ray.dot(&ref_ray);
                if shininess > 0.0 && sphere.spec > 0.0{
                    shininess /= ray.dot(ray).sqrt();
                    shininess /= ref_ray.dot(&ref_ray).sqrt();
                    shininess = shininess.powf(sphere.bright);

                    let mut spec = light.intensity.clone();
                    spec *= shininess;
                    spec *= sphere.spec;
                    color += &spec;
                }
            }
        }
        return color;
    }
    pub fn traceray(&self, origin :&Vector4, ray: &Vector4, min_t:f64, bounce_ct : i32) -> Vector4{
        let mut color = self.back_color.clone();
        if let Some((sphere, t, col_pt, normal)) = self.check_collision(&origin, &ray, min_t){
            color = Vector4::zero();
            let mut amb_color = self.amb_color.clone();
            amb_color *= sphere.amb;
            amb_color *= &sphere.color;

            color += &amb_color;

            let light_color = self.computeLightColor(&col_pt, &ray, &normal, sphere);
            color += &light_color;

            if bounce_ct > 0 {
                let normal_len_sq = normal.dot(&normal);
                let dot = 2.0 * ray.dot(&normal) / normal_len_sq;
                let mut bounce = normal.clone();
                bounce *= dot;

                let mut refl_ray = ray.clone();
                refl_ray -= &bounce;

                let mut refl_color = self.traceray(&col_pt, &refl_ray, 0.0000001, bounce_ct - 1);
                refl_color *= sphere.refl;
                color += &refl_color;
            }
        }
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
                let mut ray = Vector4::vec(x,y, -self.near); //Ray directly in the center of pixel at (x,y).
                let mut color = self.traceray(&eye, &ray, 1.0000001, 3);
                average_color += &color;

                for i in 0..extra_points{
                    let angle = 2.0 * std::f64::consts::PI * (i as f64 / extra_points as f64) + 0.25 * std::f64::consts::PI;
                    let variance_x = 0.5 + 0.45 * angle.cos();
                    let variance_y = 0.5 + 0.45 * angle.sin();
                    let x : f64 = self.left + (self.right - self.left) * ((px_x as f64 + variance_x) / self.width as f64);
                    let y : f64 = self.top - (self.top - self.bottom) * ((px_y as f64 + variance_y) / self.height as f64);

                    let mut ray = Vector4::vec(x, y, -self.near);
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
    pub fn read_resolution(tokens: &Vec<&str>) -> Result<(u32, u32), io::Error>{
        if tokens.len() != 3 {
            return Err(Error::new(ErrorKind::Other, "Incorrect format for resolution line!"));
        }

        let width_res = tokens[1].to_string().trim().parse::<u32>();
        let height_res = tokens[2].to_string().trim().parse::<u32>();
        if width_res.is_err() || height_res.is_err(){
            return Err(Error::new(ErrorKind::Other, "Please enter a positive integer width and height.")); 
        }

        let width = width_res.unwrap();
        let height = height_res.unwrap();
        if width == 0 || height == 0 {
            return Err(Error::new(ErrorKind::Other, "Please enter a positive integer width and height.")); 
        }
        return Ok((width, height));
    }
    pub fn read_scene_param(tokens: &Vec<&str>, param_name: &str, should_be_pos: bool) -> Result<f64, io::Error>{
        if tokens.len() != 2 {
            return Err(Error::new(ErrorKind::Other, format!("Incorrect format for {param_name} line!")));
        }

        let result = tokens[1].to_string().trim().parse::<f64>();
        if result.is_err(){
            return Err(Error::new(ErrorKind::Other, format!("Failed to parse value for {param_name}!"))); 
        }

        let param_val = result.unwrap();
        if (should_be_pos && param_val <= 0.0) || (!should_be_pos && param_val >= 0.0) {
            return Err(Error::new(ErrorKind::Other, format!("Incorrect sign for {param_name}!."))); 
        }
        return Ok(param_val);
    }
    pub fn read_from_file(filename: &String) -> Result<Self, io::Error>{
        let path = Path::new(&filename);
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let lines = (&mut reader).lines();
        
        let mut spheres = Vec::<Sphere>::new();
        let mut lights = Vec::<Light>::new();

        let mut near = 0.0;
        let (mut left, mut right, mut bottom, mut top) : (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);
        let (mut width, mut height): (u32, u32) = (0, 0); 
        let mut back_color = Vector4::zero();
        let mut amb_color = Vector4::zero();
        let mut output_file = "output.txt".to_string();
        for line in lines.map_while(Result::ok){
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let first_token = tokens[0];
            match first_token {
                "SPHERE" => {
                    let result = Sphere::read_from_tokens(&tokens);
                    if result.is_none() {
                        return Err(Error::new(ErrorKind::Other, format!("Could not read sphere from {line}")));
                    }
                    spheres.push(result.unwrap());
                },
                "LIGHT" => {
                    let result = Light::read_from_tokens(&tokens);
                    if result.is_none() {
                        return Err(Error::new(ErrorKind::Other, format!("Could not read light from {line}")));
                    }
                    lights.push(result.unwrap());
                    
                },
                "RES" => {
                    if width != 0 {
                        return Err(Error::new(ErrorKind::Other, "Only one line for the resolution is permitted."));
                    }
                    let result = Self::read_resolution(&tokens);
                    if let Err(e) = result {
                        return Err(e);
                    }
                    (width, height) = result.unwrap();
                },
                "NEAR" => {
                    if near != 0.0{
                        return Err(Error::new(ErrorKind::Other, "Only one line for the near plane is permitted."));
                    }
                    let result = Self::read_scene_param(&tokens, "near", true);
                    if let Err(e) = result {
                        return Err(e);
                    }
                    near = result.unwrap();
                },
                "TOP" => top = tokens[1].to_string().trim().parse::<f64>().expect("Please enter a float."),
                "BOTTOM" => bottom = tokens[1].to_string().trim().parse::<f64>().expect("Please enter a float."),
                "LEFT" => left = tokens[1].to_string().trim().parse::<f64>().expect("Please enter a float."),
                "RIGHT" => right = tokens[1].to_string().trim().parse::<f64>().expect("Please enter a float."),
                "BACK" => {
                    if tokens.len() != 4 {
                        return Err(Error::new(ErrorKind::Other, "Unrecognized token in file!"));
                    }
                    let back_color_opt = Vector4::vec_from_str_slice(&tokens[1..4]);
                    if back_color_opt.is_none() {
                        return Err(Error::new(ErrorKind::Other, "Unrecognized token in file!"));
                    }
                    back_color = back_color_opt.unwrap();
                },
                "AMBIENT" =>{
                    if tokens.len() != 4 {
                        return Err(Error::new(ErrorKind::Other, "Unrecognized token in file!"));
                    }
                    let amb_color_opt = Vector4::vec_from_str_slice(&tokens[1..4]);
                    if amb_color_opt.is_none(){
                        return Err(Error::new(ErrorKind::Other, "Unrecognized token in file!"));
                    }
                    amb_color = amb_color_opt.unwrap();
                } 
                "OUTPUT" => output_file = tokens[1].to_string().trim().to_string(),
                &_ => {
                    return Err(Error::new(ErrorKind::Other, "Unrecognized token in file!"));
                }
            }
        }
        //TODO error handling....
        let capacity = (width * height) as usize;
        let array : Vec<Vector4> = vec![back_color.clone(); capacity];
        return Ok(Self{near, left, right, bottom, top, width, height, 
            spheres, lights, back_color, amb_color, array, output_file});
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
        write!(f, "\nOutput filename: {}", self.output_file)?;
        return Ok(());
    }
}