use std::fmt;
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
                Err(_e) => return None,
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

pub trait Shape{
    fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64, max:f64, print_check : bool) -> Option<(f64,Vector4,Vector4)>;
    fn lighting_params(&self) -> (&Vector4, f64, f64, f64, f64);
    fn refl(&self) -> f64; 
}

pub struct Cube{
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


impl Cube {
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
                Err(_e) => return None,
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
                Err(_e) => return None,
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
impl Shape for Cube{
    fn lighting_params(&self) -> (&Vector4, f64, f64, f64, f64){
        return (&self.color, self.amb, self.diff, self.spec, self.bright);
    }
    fn refl(&self) -> f64{
        return self.refl;
    }
    fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64, mut max: f64, _print_check : bool) -> Option<(f64,Vector4,Vector4)>{
        let origin_prime = &self.inv_matrix * origin;
        let ray_prime = &self.inv_matrix * ray;
        let mut col_data = None;
        let normals = vec![Vector4::vec(-1.0,0.0,0.0), Vector4::vec(1.0,0.0,0.0), Vector4::vec(0.0,-1.0,0.0), Vector4::vec(0.0,1.0,0.0), Vector4::vec(0.0,0.0,-1.0), Vector4::vec(0.0,0.0,1.0)];

        for i in 0..6{
            let ray_proj = ray_prime.dot(&normals[i]);
            let origin_proj = origin_prime.dot(&normals[i]);
            let surface_proj = 1.0;
            let distance = surface_proj - origin_proj;

            let t = distance / ray_proj;
            if t <= min || t >= max{
                continue;
            }
            
            let mut col_pt_prime = ray_prime.clone();
            col_pt_prime *= t;
            col_pt_prime += &origin_prime;

            let (dim_one, dim_two) = match i {
                0 | 1 => (col_pt_prime.y(), col_pt_prime.z()),
                2 | 3  => (col_pt_prime.x(), col_pt_prime.z()),
                _ => (col_pt_prime.x(), col_pt_prime.y()),
            };
            if -1.0 <= dim_one && dim_one <= 1.0 && -1.0 <= dim_two && dim_two <= 1.0{
                max = t;
                let normal_copy : Vector4 = match ray_proj >= 0.0{
                    true => {
                        match i % 2 {
                            0 => normals[i + 1].clone(),
                            _ => normals[i - 1].clone(),
                        }
                    },
                    false => normals[i].clone(),
                };
                
                let mut col_pt = ray.clone();
                col_pt *= t;
                col_pt += &origin;
                col_pt.force_point(); 
                let mut normal = &self.inv_transp * &normal_copy;
                normal.force_vec();
                
                col_data = Some((t, col_pt, normal));
            }
        }
        return col_data;
    }
}
impl Shape for Sphere{
    fn lighting_params(&self) -> (&Vector4, f64, f64, f64, f64){
        return (&self.color, self.amb, self.diff, self.spec, self.bright);
    }
    fn refl(&self) -> f64{
        return self.refl;
    }
    fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64, max: f64, _print_check: bool) -> Option<(f64,Vector4,Vector4)>{
        let origin_prime = &self.inv_matrix * origin;
        let ray_prime = &self.inv_matrix * ray;
        
        let a = ray_prime.dot(&ray_prime);
        let b = origin_prime.dot(&ray_prime);
        let c = origin_prime.dot(&origin_prime) - 1.0;

        let det = b * b - a * c;
        if det >= 0.0{
            let sqrt_det = det.sqrt();
            let mut t = (-b - sqrt_det) / a;
            //Ray missed the front of the sphere.
            //Try for a collision for the back of the sphere.
            if t < min { 
                t = (-b + sqrt_det) / a;
            }
            //If there is a collision between the min and max...
            if t > min && t < max{
                //Calculate the collision point on sphere...
                let mut col_pt = ray.clone();
                col_pt *= t;
                col_pt += &origin;
                col_pt.force_point();
    
                //Calculate the normal of collision...
                let mut col_pt_prime = ray_prime.clone();
                col_pt_prime *= t;
                col_pt_prime += &origin_prime;
    
                if ray_prime.dot(&ray_prime) > origin_prime.dot(&origin_prime) {
                    col_pt_prime *= -1.0;
                }
                col_pt_prime.force_vec();
                let mut normal =  &self.inv_transp * &col_pt_prime;
                normal.force_vec();

                //TODO force len of normal to 1.0?
                return Some((t, col_pt, normal));
            }
        }
        return None;
    }
}
impl fmt::Display for Cube{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cube with scale {}, rotation X:{} Y:{} Z:{} and color {} located at {}.\n", self.scale, self.r_x, self.r_y, self.r_z, self.color, self.pos)?;
        return write!(f, "Lighting coefficients: amb:{} diff:{} spec:{} refl:{} bright:{}.", self.amb, self.diff, self.spec, self.refl, self.bright);
    }
}
impl fmt::Display for Sphere{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere with scale {}, rotation X:{} Y:{} Z:{} and color {} located at {}.\n", self.scale, self.r_x, self.r_y, self.r_z, self.color, self.pos)?;
        return write!(f, "Lighting coefficients: amb:{} diff:{} spec:{} refl:{} bright:{}.", self.amb, self.diff, self.spec, self.refl, self.bright);
    }
}

