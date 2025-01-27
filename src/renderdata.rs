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

    pub fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64) -> Option<(&Sphere,f64)> {
        let mut lowest = std::f64::INFINITY;
        let mut s : Option<(&Sphere,f64)> = None;
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
                    s = Some((sphere,t));
                    lowest = t;
                }
            }
        }
        return s;
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
            if let Some((dummy,t)) = result{
                if t > 1.0{ //If there is a collision that occurs after the light, we don't care...
                    result = None;
                }
            }
            if result.is_none() {
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
        if let Some((sphere,t)) = self.check_collision(&origin, &ray, min_t){
            let mut col_pt = ray.clone();
            col_pt *= t;
            col_pt += &origin;

            let origin_prime = &sphere.inv_matrix * origin;
            let mut col_pt_prime = origin_prime.clone();
            let mut ray_prime = &sphere.inv_matrix * ray;
            ray_prime *= t;
            col_pt_prime += &ray_prime;

            if ray_prime.dot(&ray_prime) > origin_prime.dot(&origin_prime) {
                col_pt_prime *= -1.0;
            }
            col_pt_prime.force_vec();
            let mut normal =  &sphere.inv_transp * &col_pt_prime;
            normal.force_vec();

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
    pub fn render(&mut self){
        let eye = Vector4::point(0.0,0.0,0.0);
        for px_y in 0..self.height{
            for px_x in 0..self.width{
                let x : f64 = self.left + (self.right - self.left) * ((px_x as f64 + 0.5) / self.width as f64);
                let y : f64 = self.top - (self.top - self.bottom) * ((px_y as f64 + 0.5) / self.height as f64);

                let mut ray = Vector4::vec(x,y, -self.near); //Ray directly in the center of pixel at (x,y).
                let color = self.traceray(&eye, &ray, 1.0000001, 3);

                self.array[(px_y * self.height + px_x) as usize] = color;
            }
        }

    }
    pub fn read_from_file(filename: &String) -> Result<Self, io::Error>{
        let path = Path::new(&filename);
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let lines = (&mut reader).lines();
        
        let mut spheres = Vec::<Sphere>::new();
        let mut lights = Vec::<Light>::new();

        let mut near = 1.0;
        let (mut left, mut right, mut bottom, mut top) : (f64, f64, f64, f64) = (-1.0, 1.0, -1.0, 1.0);
        let (mut width, mut height): (u32, u32) = (800, 600); 
        let mut back_color = Vector4::zero();
        let mut amb_color = Vector4::zero();
        let mut output_file = "output.txt".to_string();
        for line in lines.map_while(Result::ok){
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let first_token = tokens[0];
            match first_token {
                "SPHERE" => {
                    let new_sphere = Sphere::read_from_tokens(&tokens); //TODO error handling
                    spheres.push(new_sphere);
                },
                "LIGHT" => {
                    let light = Light::read_from_tokens(&tokens); //TODO error handling
                    lights.push(light);
                },
                "RES" => {
                    width = tokens[1].to_string().trim().parse::<u32>().expect("Please enter an integer width.");
                    height = tokens[2].to_string().trim().parse::<u32>().expect("Please enter an integer height.");
                },
                "NEAR" => near = tokens[1].to_string().trim().parse::<f64>().expect("Please enter a float."),
                "TOP" => top = tokens[1].to_string().trim().parse::<f64>().expect("Please enter a float."),
                "BOTTOM" => bottom = tokens[1].to_string().trim().parse::<f64>().expect("Please enter a float."),
                "LEFT" => left = tokens[1].to_string().trim().parse::<f64>().expect("Please enter a float."),
                "RIGHT" => right = tokens[1].to_string().trim().parse::<f64>().expect("Please enter a float."),
                "BACK" => back_color = Vector4::from_slice(&tokens[1..4]),
                "AMBIENT" => amb_color = Vector4::from_slice(&tokens[1..4]),
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