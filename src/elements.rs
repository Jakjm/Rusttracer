use std::fmt;
use std::io;
//mod matrix;
use crate::matrix::Vector4;
use crate::matrix::Matrix4;

pub struct Color{
    red: f64,
    green: f64, 
    blue: f64,
}

impl Color{
    pub fn new(red: f64, green: f64, blue: f64) -> Self{
        return Self{red, green, blue};
    }
    pub fn from_slice(slice: &[&str]) -> Self{
        let red = slice[0].to_string().trim().parse::<f64>().expect("Please enter an intensity between 0.0 and 1.0.");
        let green = slice[1].to_string().trim().parse::<f64>().expect("Please enter an intensity between 0.0 and 1.0.");
        let blue = slice[2].to_string().trim().parse::<f64>().expect("Please enter an intensity between 0.0 and 1.0.");
        return Self{red, green, blue};
    }
}

pub struct Light{
    pos: Vector4,
}
impl Light{
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Self{
        let x = tokens[2].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let y = tokens[3].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let z = tokens[4].to_string().trim().parse::<f64>().expect("Please enter a float.");
        return Self{pos: Vector4::point(x,y,z)};
    }
}

pub struct Sphere{
    pos: Vector4,
    scale: Vector4,
    matrix: Matrix4,
    color: Color,
    amb: f64,
    diff: f64,
    spec: f64,
    refl: f64,
    bright: f64,
}
impl Sphere{
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Self{
        
        let x = tokens[2].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let y = tokens[3].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let z = tokens[4].to_string().trim().parse::<f64>().expect("Please enter a float.");
        
        let scale_x = tokens[5].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let scale_y = tokens[6].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let scale_z = tokens[7].to_string().trim().parse::<f64>().expect("Please enter a float.");

        let trans_matrix = Matrix4::trans(x,y,z);
        let scale_matrix = Matrix4::scale(x,y,z);
        let matrix = &trans_matrix * &scale_matrix;
        
        let color = Color::from_slice(&tokens[8..11]);

        return Self{pos: Vector4::point(x,y,z), scale: Vector4::vec(scale_x, scale_y, scale_y), matrix,
            color, amb: 0.0, diff: 0.0, spec: 0.0, refl: 0.0, bright: 1.0};
    }
}
impl fmt::Display for Sphere{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //return 
        return write!(f, "Sphere located at {}", self.pos);
    }
}

