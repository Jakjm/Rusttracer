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
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Option<Self>{
        let pos_opt = Vector4::point_from_slice(&tokens[2..5]);
        let intensity_opt = Vector4::vec_from_slice(&tokens[5..8]);
        if let Some(pos) = pos_opt{
            if let Some(intensity) = intensity_opt{
                return Some(Self{pos, intensity});
            }
        }
        return None;
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
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Option<Self>{
        let pos_opt = Vector4::point_from_slice(&tokens[2..5]);
        let scale_opt = Vector4::vec_from_slice(&tokens[5..8]);
        let color_opt = Vector4::vec_from_slice(&tokens[8..11]);
        if let Some(pos) = pos_opt {
            if let Some(scale) = scale_opt {
                if let Some(color) = color_opt {
                    let trans_matrix = Matrix4::trans(pos.x(),pos.y(),pos.z());
                    let scale_matrix = Matrix4::scale(scale.x(),scale.y(),scale.z());
                    let inv_matrix = &trans_matrix * &scale_matrix;
                    let inv_matrix = inv_matrix.inverse();
                    let inv_transp = inv_matrix.transpose();
                    

                    let amb = tokens[11].to_string().trim().parse::<f64>().unwrap();
                    let diff = tokens[12].to_string().trim().parse::<f64>().unwrap();
                    let spec = tokens[13].to_string().trim().parse::<f64>().unwrap();
                    let refl = tokens[14].to_string().trim().parse::<f64>().unwrap();
                    let bright = tokens[15].to_string().trim().parse::<f64>().unwrap();

                    return Some(Self{pos, scale, inv_matrix, inv_transp,color, amb, diff, spec, refl, bright});
                } 
            }
        }
        return None;
    }
}
impl fmt::Display for Sphere{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere with scale {} and color {} located at {}.\n", self.scale, self.color, self.pos)?;
        return write!(f, "Lighting coefficients: amb:{} diff:{} spec:{} refl:{} bright:{}.", self.amb, self.diff, self.spec, self.refl, self.bright);
    }
}

