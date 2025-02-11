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
        if tokens.len() != 8{
            return None;
        }

        //Parse the tokens into f64s...
        let token_slice: &[&str] = &tokens[2..];
        let mut parsed_tokens: [f64;6] = [0.0;6];
        for i in 0..token_slice.len(){
            let parse_result = token_slice[i].to_string().trim().parse::<f64>();
            match parse_result{
                Err(e) => return None,
                Ok(num) => parsed_tokens[i] = num,
            }
        }
        let pos = Vector4::point_from_slice(&parsed_tokens[0..3]);
        let intensity = Vector4::vec_from_slice(&parsed_tokens[3..6]);
        return Some(Self{pos, intensity});
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
    pub r_x: f64, 
    pub r_y: f64,
    pub r_z: f64,
    pub color: Vector4,
    pub amb: f64,
    pub diff: f64,
    pub spec: f64,
    pub refl: f64,
    pub bright: f64,
}
impl Sphere{
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Option<Self>{
        if tokens.len() < 16 || tokens.len() > 19 {
            return None
        }

        //Parse the tokens into f64s...
        let token_slice: &[&str] = &tokens[2..];
        let mut parsed_tokens: [f64;17] = [0.0;17];
        for i in 0..token_slice.len(){
            let parse_result = token_slice[i].to_string().trim().parse::<f64>();
            match parse_result{
                Err(e) => return None,
                Ok(num) => parsed_tokens[i] = num,
            }
        }

        let pos = Vector4::point_from_slice(&parsed_tokens[0..3]);
        let scale = Vector4::vec_from_slice(&parsed_tokens[3..6]);
        let mut r_x: f64 = 0.0;
        let mut r_y: f64 = 0.0;
        let mut r_z: f64 = 0.0;
        let mut color_start: usize = 6;
        let mut lighting_param_start: usize = 9;

        if tokens.len() > 16 {
            r_x = parsed_tokens[6];
            color_start += 1;
            lighting_param_start += 1;
        }
        if tokens.len() > 17 {
            r_y = parsed_tokens[7];
            color_start += 1;
            lighting_param_start += 1;
        }
        if tokens.len() > 18 {
            r_z = parsed_tokens[8];
            color_start += 1;
            lighting_param_start += 1;
        }
        let color = Vector4::vec_from_slice(&parsed_tokens[color_start..lighting_param_start]);
        
        let trans_matrix = Matrix4::trans(pos.x(),pos.y(),pos.z());
        let scale_matrix = Matrix4::scale(scale.x(),scale.y(),scale.z());
        let rot_x_matrix = Matrix4::rot_x(r_x);
        let rot_y_matrix = Matrix4::rot_y(r_y);
        let rot_z_matrix = Matrix4::rot_z(r_z);
        
        let rotation_matrix = &rot_z_matrix * &(&rot_y_matrix * &rot_x_matrix);
        let inv_matrix = &trans_matrix * &(&rotation_matrix * &scale_matrix);
        let inv_matrix = inv_matrix.inverse();
        let inv_transp = inv_matrix.transpose();

        let lighting_values = &parsed_tokens[lighting_param_start..];
        let amb = lighting_values[0];
        let diff = lighting_values[1];
        let spec = lighting_values[2];
        let refl = lighting_values[3];
        let bright = lighting_values[4];
        return Some(Self{pos, scale, inv_matrix, inv_transp, r_x, r_y, r_z, color, amb, diff, spec, refl, bright});
    }
}
impl fmt::Display for Sphere{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere with scale {}, rotation X:{} Y:{} Z:{} and color {} located at {}.\n", self.scale, self.r_x, self.r_y, self.r_z, self.color, self.pos)?;
        return write!(f, "Lighting coefficients: amb:{} diff:{} spec:{} refl:{} bright:{}.", self.amb, self.diff, self.spec, self.refl, self.bright);
    }
}

