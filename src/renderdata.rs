use std::env;
use std::fmt;
use std::io;
use std::io::{Error, ErrorKind};
use std::fs::File;
use std::io::{BufRead, BufReader};
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
    back_red: f64,
    back_green: f64,
    back_blue: f64,
    amb_red_intensity: f64,
    amb_green_intensity: f64,
    amb_blue_intensity: f64,
    output_file: String,
}

impl RenderData{
    pub fn read_from_file(filename: &String) -> Result<Self, io::Error>{
        let path = Path::new(&filename);
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let lines = (&mut reader).lines();
        
        let mut spheres = Vec::new();
        let mut lights = Vec::new();

        let mut near = 1.0;
        let (mut left, mut right, mut bottom, mut top) : (f64, f64, f64, f64) = (-1.0, 1.0, -1.0, 1.0);
        let (mut width, mut height): (u32, u32) = (800, 600); 
        let (mut back_red, mut back_green, mut back_blue) : (f64, f64, f64) = (0.0, 0.0, 0.0);
        let (mut amb_red_intensity, mut amb_green_intensity, mut amb_blue_intensity) : (f64, f64, f64) = (0.0, 0.0, 0.0);
        let mut output_file = "output.txt".to_string();
        for line in lines.map_while(Result::ok){
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let first_token = tokens[0];
            match first_token {
                "SPHERE" => {
                    let new_sphere = Sphere::read_from_tokens(&tokens);
                    spheres.push(new_sphere);
                },
                "LIGHT" => {
                    let light = Light::read_from_tokens(&tokens);
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
                "BACK" =>{
                    back_red = tokens[1].to_string().trim().parse::<f64>().expect("Please enter an intensity between 0.0 and 1.0.");
                    back_green = tokens[2].to_string().trim().parse::<f64>().expect("Please enter an intensity between 0.0 and 1.0.");
                    back_blue = tokens[3].to_string().trim().parse::<f64>().expect("Please enter an intensity between 0.0 and 1.0.");
                }
                "AMBIENT" => {
                    amb_red_intensity = tokens[1].to_string().trim().parse::<f64>().expect("Please enter an intensity between 0.0 and 1.0.");
                    amb_green_intensity = tokens[2].to_string().trim().parse::<f64>().expect("Please enter an intensity between 0.0 and 1.0.");
                    amb_blue_intensity = tokens[3].to_string().trim().parse::<f64>().expect("Please enter an intensity between 0.0 and 1.0.");
                }
                "OUTPUT" => output_file = tokens[1].to_string().trim().to_string(),
                &_ => {
                    return Err(Error::new(ErrorKind::Other, "Unrecognized token in file!"));
                }
            }
        }

        //
        return Ok(Self{near: near, left: left, right: right, bottom:bottom, top: top, width: width, 
            height: height, spheres, lights, back_red, back_green, back_blue,
            amb_red_intensity: amb_red_intensity, amb_green_intensity : amb_green_intensity, amb_blue_intensity : amb_blue_intensity, output_file : output_file});
    }
}

impl fmt::Display for RenderData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //return 
        write!(f, "Scene Resolution: {}x{} pixels\n", self.width, self.height)?;
        write!(f, "Shapes:\n")?;
        for sp in self.spheres.iter() {
            write!(f, "{sp}\n")?;
        }
        write!(f, "Near plane: {}, X range: {{{},{}}} Y range: {{{},{}}}", self.near, self.left, self.right, self.bottom, self.top)?;
        return Ok(());
    }
}