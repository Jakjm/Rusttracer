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
        let mut back_color = Color::new(0.0, 0.0, 0.0);
        let mut amb_color = Color::new(0.0, 0.0, 0.0);
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
                "BACK" => back_color = Color::from_slice(&tokens[1..4]),
                "AMBIENT" => amb_color = Color::from_slice(&tokens[1..4]),
                "OUTPUT" => output_file = tokens[1].to_string().trim().to_string(),
                &_ => {
                    return Err(Error::new(ErrorKind::Other, "Unrecognized token in file!"));
                }
            }
        }

        return Ok(Self{near, left, right, bottom, top, width, height, 
            spheres, lights, back_color, amb_color, output_file});
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