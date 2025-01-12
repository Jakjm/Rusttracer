use std::fmt;
use std::io;
//mod matrix;
use crate::matrix::Vector4;
use crate::matrix::Matrix4;
pub struct Sphere{
    pos: Vector4,
    color: u32,
    amb: f64,
    diff: f64,
    spec: f64,
    refl: f64,
    bright: f64,
}
impl Sphere{
    pub fn readFromString(input: String) -> Self{
        let tokens: Vec<&str> = input.split_whitespace().collect();
        let x = tokens[2].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let y = tokens[3].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let z = tokens[4].to_string().trim().parse::<f64>().expect("Please enter a float.");
        return Self{pos: Vector4::point(x,y,z), color: 0, amb: 0.0, diff: 0.0, spec: 0.0, refl: 0.0, bright: 1.0};
    }
}
impl fmt::Display for Sphere{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //return 
        return write!(f, "Sphere located at {}", self.pos);
    }
}