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
        if tokens.len() == 8{
            if let Some(pos) = pos_opt{
                if let Some(intensity) = intensity_opt{
                    return Some(Self{pos, intensity});
                }
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
    pub rX: f64, 
    pub rY: f64,
    pub color: Vector4,
    pub amb: f64,
    pub diff: f64,
    pub spec: f64,
    pub refl: f64,
    pub bright: f64,
}
impl Sphere{
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Option<Self>{
        if tokens.len() == 16 || tokens.len() == 18{
            let pos_opt = Vector4::point_from_slice(&tokens[2..5]);
            let scale_opt = Vector4::vec_from_slice(&tokens[5..8]);
            let mut color_opt: Option<Vector4>; 
            let mut rX: f64 = 0.0;
            let mut rY: f64 = 0.0;
            if tokens.len() == 16 {
                color_opt = Vector4::vec_from_slice(&tokens[8..11]);
            }
            else{
                match tokens[8].to_string().trim().parse::<f64>(){
                    Ok(num) => rX = num,
                    Err(e) => return None,
                }
                match tokens[9].to_string().trim().parse::<f64>(){
                    Ok(num) => rY = num,
                    Err(e) => return None,
                }
                color_opt = Vector4::vec_from_slice(&tokens[10..13]);
               
            }
            if let Some(pos) = pos_opt {
                if let Some(scale) = scale_opt {
                    if let Some(color) = color_opt {
                        let trans_matrix = Matrix4::trans(pos.x(),pos.y(),pos.z());
                        let scale_matrix = Matrix4::scale(scale.x(),scale.y(),scale.z());
                        let inv_matrix = &trans_matrix * &scale_matrix;
                        let inv_matrix = inv_matrix.inverse();
                        let inv_transp = inv_matrix.transpose();
                        
                        //let mut lighting_values: [Result<f64, ParseFloatError>; 5] = [Ok(0.0); 5];
                        let mut lighting_values: [f64; 5] = [0.0; 5];
                        for i in 0..5{
                            let parse_result = tokens[11 + i].to_string().trim().parse::<f64>();
                            match parse_result{
                                Err(e) => return None,
                                Ok(num) => lighting_values[i] = num,
                            }
                        }
                        let amb = lighting_values[0];
                        let diff = lighting_values[1];
                        let spec = lighting_values[2];
                        let refl = lighting_values[3];
                        let bright = lighting_values[4];
                        

                        return Some(Self{pos, scale, inv_matrix, inv_transp, rX, rY, color, amb, diff, spec, refl, bright});
                    } 
                }
            }
        }
        return None;
    }
}
impl fmt::Display for Sphere{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere with scale {}, rotation X:{} Y:{} and color {} located at {}.\n", self.scale, self.rX, self.rY, self.color, self.pos)?;
        return write!(f, "Lighting coefficients: amb:{} diff:{} spec:{} refl:{} bright:{}.", self.amb, self.diff, self.spec, self.refl, self.bright);
    }
}

