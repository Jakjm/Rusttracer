use std::fmt;
use std::io;
//mod matrix;
use crate::matrix::Vector4;
use crate::matrix::Matrix4;

pub struct Light{
    pub pos: Vector4,
    pub intensity: Vector4,
}
impl Light{
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Self{
        let x = tokens[2].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let y = tokens[3].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let z = tokens[4].to_string().trim().parse::<f64>().expect("Please enter a float.");

        let intensity = Vector4::from_slice(&tokens[5..8]);
        return Self{pos: Vector4::point(x,y,z), intensity};
    }
}

impl fmt::Display for Light{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Light with color {} located at {}.", self.intensity, self.pos);
    }
}

pub struct Sphere{
    pos: Vector4,
    scale: Vector4,
    pub inv_matrix: Matrix4,
    pub inv_transp: Matrix4,
    pub color: Vector4,
    pub amb: f64,
    pub diff: f64,
    pub spec: f64,
    pub refl: f64,
    pub bright: f64,
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
        let inv_matrix = &trans_matrix * &scale_matrix;
        let inv_matrix = inv_matrix.inverse();
        let inv_transp = inv_matrix.transpose();
        let color = Vector4::from_slice(&tokens[8..11]);

        let amb = tokens[11].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let diff = tokens[12].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let spec = tokens[13].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let refl = tokens[14].to_string().trim().parse::<f64>().expect("Please enter a float.");
        let bright = tokens[15].to_string().trim().parse::<f64>().expect("Please enter a float.");

        return Self{pos: Vector4::point(x,y,z), scale: Vector4::vec(scale_x, scale_y, scale_z), inv_matrix, inv_transp,
            color, amb, diff, spec, refl, bright};
    }
}
impl fmt::Display for Sphere{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere with scale {} and color {} located at {}.\n", self.scale, self.color, self.pos)?;
        return write!(f, "Lighting coefficients: amb:{} diff:{} spec:{} refl:{} bright:{}.", self.amb, self.diff, self.spec, self.refl, self.bright);
    }
}

