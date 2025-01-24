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
use crate::elements::Color;
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
    back_color: Color,
    amb_color: Color,
    array: Vec<Color>,
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

    pub fn check_collision(&self, vec: &Vector4, pt: &Vector4, min: f64) -> Option<(&Sphere,f64)> {
        let mut s : Option<(&Sphere,f64)> = None;
        for sphere in self.spheres.iter(){
            let vec_prime = &sphere.matrix * vec;
            let pt_prime = &sphere.matrix * pt;
            let a = vec_prime.dot(&vec_prime);
            let b = pt_prime.dot(&vec_prime);
            let c = pt_prime.dot(&pt_prime) - 1.0;

            let det = b * b - a * c;
            if det >= 0.0{
                let sqrt_det = det.sqrt();
                let tOne = (-b - sqrt_det) / a;
                if tOne > min{
                    s = Some((sphere,tOne));
                }
                else{
                    let tTwo = (-b + sqrt_det) / a;
                    if tTwo > min{
                        s = Some((sphere,tTwo));
                    }
                }
            }
        }
        return s;
    }
    pub fn traceray(&mut self, origin :&Vector4, ray: &mut Vector4, min_t:f64, bounceCt : i32) -> Color{
        let mut color = self.back_color.clone();
        if let Some((sphere,t)) = self.check_collision(&origin, &ray, min_t){
            *ray *= t;
            let mut colPt = origin.clone();
            colPt += &ray;
            color = Color::new(0.0,0.0,1.0);
            println!("collision!");
        }
        return color;
    }
    pub fn render(&mut self){
        let eye = Vector4::point(0.0,0.0,0.0);
        for px_y in 0..self.height{
            for px_x in 0..self.width{
                let x : f64 = self.left + (self.right - self.left) * (px_x as f64 / self.width as f64);
                let y : f64 = self.top - (self.top - self.bottom) * (px_y as f64 / self.height as f64);

                let mut ray = Vector4::vec(x,y, -self.near);
                let color = self.traceray(&eye, &mut ray, 1.0, 3);
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
        let mut back_color = Color::new(0.0, 0.0, 0.0);
        let mut amb_color = Color::new(0.0, 0.0, 0.0);
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
                "BACK" => back_color = Color::from_slice(&tokens[1..4]),
                "AMBIENT" => amb_color = Color::from_slice(&tokens[1..4]),
                "OUTPUT" => output_file = tokens[1].to_string().trim().to_string(),
                &_ => {
                    return Err(Error::new(ErrorKind::Other, "Unrecognized token in file!"));
                }
            }
        }
        //TODO error handling....
        let capacity = (width * height) as usize;
        let array : Vec<Color> = vec![back_color.clone(); capacity];
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