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

impl Clone for Color{
    fn clone(&self) -> Self{
        return Self{red: self.red, green : self.green, blue: self.blue};
    }
}
impl Color{
    pub fn to_rgb(&self) -> (u8, u8, u8){
        return ((255.0 * self.red) as u8, (255.0 * self.green) as u8, (255.0 * self.blue) as u8);
    }
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

impl fmt::Display for Color{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //return 
        return write!(f, "(R:{} G:{} B:{})", self.red, self.green, self.blue);
    }
}

pub struct Light{
    pos: Vector4,
    color: Color,
}
impl Light{
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Self{
        let x = tokens[2].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let y = tokens[3].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let z = tokens[4].to_string().trim().parse::<f64>().expect("Please enter a float.");

        let color = Color::from_slice(&tokens[5..8]);
        return Self{pos: Vector4::point(x,y,z), color};
    }
}

impl fmt::Display for Light{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Light with color {} located at {}.", self.color, self.pos);
    }
}

pub struct Sphere{
    pos: Vector4,
    scale: Vector4,
    pub matrix: Matrix4,
    pub color: Color,
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
        let scale_matrix = Matrix4::scale(scale_x,scale_y,scale_z);
        let matrix = &trans_matrix * &scale_matrix;
        let matrix = matrix.inverse();
        
        let color = Color::from_slice(&tokens[8..11]);

        let amb = tokens[11].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let diff = tokens[12].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let spec = tokens[13].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let refl = tokens[14].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let bright = tokens[15].to_string().trim().parse::<f64>().expect("Please enter a float.");

        return Self{pos: Vector4::point(x,y,z), scale: Vector4::vec(scale_x, scale_y, scale_z), matrix,
            color, amb, diff, spec, refl, bright};
    }
}
impl fmt::Display for Sphere{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere with scale {} and color {} located at {}.\n", self.scale, self.color, self.pos)?;
        return write!(f, "Lighting coefficients: amb:{} diff:{} spec:{} refl:{} bright:{}.", self.amb, self.diff, self.spec, self.refl, self.bright);
    }
}

