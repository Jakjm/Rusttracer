use std::fmt;
use crate::matrix::Vector4;
//use crate::matrix::Matrix4;
use crate::shape::{LightingProps,Shape};
use crate::polygon::{SpatialProps,Polygon};
use crate::mesh_shape::MeshShape;

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

pub struct Dodecahedron{
    pub mesh_shape: MeshShape,
}
impl Dodecahedron {
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Option<Self>{
        return match MeshShape::read_from_tokens(tokens){
            Some((spatial_props, lighting_props)) => {
            // The golden ratio, φ, and its reciprocal are used to define the vertices.
            const PHI: f64 = 1.618033988749894848204586834365638118_f64;
            const INV_PHI: f64 = 1.0 / PHI;

            // The 20 unique vertices of a dodecahedron.
            // let points = vec![
            //     Vector4::point( 1.0,  1.0,  1.0),
            //     Vector4::point( 1.0,  1.0, -1.0),
            //     Vector4::point( 1.0, -1.0,  1.0),
            //     Vector4::point( 1.0, -1.0, -1.0),
            //     Vector4::point(-1.0,  1.0,  1.0),
            //     Vector4::point(-1.0,  1.0, -1.0),
            //     Vector4::point(-1.0, -1.0,  1.0),
            //     Vector4::point(-1.0, -1.0, -1.0),
            //     Vector4::point(0.0,  INV_PHI,  PHI),
            //     Vector4::point(0.0,  INV_PHI, -PHI),
            //     Vector4::point(0.0, -INV_PHI,  PHI),
            //     Vector4::point(0.0, -INV_PHI, -PHI),
            //     Vector4::point( INV_PHI,  PHI, 0.0),
            //     Vector4::point( INV_PHI, -PHI, 0.0),
            //     Vector4::point(-INV_PHI,  PHI, 0.0),
            //     Vector4::point(-INV_PHI, -PHI, 0.0),
            //     Vector4::point( PHI, 0.0,  INV_PHI),
            //     Vector4::point( PHI, 0.0, -INV_PHI),
            //     Vector4::point(-PHI, 0.0,  INV_PHI),
            //     Vector4::point(-PHI, 0.0, -INV_PHI),
            // ];

            // /// A list of 12 faces, with each face containing the explicit coordinates of its 5 vertices.
            // /// The vertices for each face are ordered counter-clockwise when viewed from outside.
            // let polygons  = vec![
            //     Polygon::new(vec![points[0], points[12], points[14], points[4], points[8]], &spatial_props), 
            //     Polygon::new(vec![points[0], points[8],  points[10], points[2], points[16]], &spatial_props),
            //     Polygon::new(vec![points[0], points[16], points[17], points[1], points[12]], &spatial_props),
            //     Polygon::new(vec![points[1], points[9],  points[11], points[3], points[17]], &spatial_props),
            //     Polygon::new(vec![points[1], points[17], points[16], points[2], points[13]], &spatial_props),  
            //     Polygon::new(vec![points[0], points[8], points[10], points[2], points[16]], &spatial_props),
            //     Polygon::new(vec![points[0], points[16], points[17], points[1], points[12]],&spatial_props),
            //     Polygon::new(vec![points[0], points[12], points[14], points[4], points[8]],&spatial_props),
            //     Polygon::new(vec![points[3], points[11], points[9], points[1], points[17]],&spatial_props),
            //     Polygon::new(vec![points[3], points[17], points[16], points[2], points[13]],&spatial_props),
            //     Polygon::new(vec![points[3], points[13], points[15], points[7], points[11]],&spatial_props),
            //     Polygon::new(vec![points[5], points[14], points[12], points[1], points[9]],&spatial_props),
            //     Polygon::new(vec![points[5], points[9], points[11], points[7], points[19]],&spatial_props),
            //     Polygon::new(vec![points[5], points[19], points[18], points[4], points[14]],&spatial_props),
            //     Polygon::new(vec![points[6], points[15], points[13], points[2], points[10]],&spatial_props),
            //     Polygon::new(vec![points[6], points[10], points[8], points[4], points[18]],&spatial_props),
            //     Polygon::new(vec![points[6], points[18], points[19], points[7], points[15]],&spatial_props),
            // ];

            let points = vec![
                Vector4::point( 1.0,  1.0,  1.0),
                Vector4::point( 1.0,  1.0, -1.0),
                Vector4::point( 1.0, -1.0,  1.0),
                Vector4::point( 1.0, -1.0, -1.0),
                Vector4::point(-1.0,  1.0,  1.0),
                Vector4::point(-1.0,  1.0, -1.0),
                Vector4::point(-1.0, -1.0,  1.0),
                Vector4::point(-1.0, -1.0, -1.0),
                Vector4::point(0.0,  INV_PHI,  PHI),
                Vector4::point(0.0,  INV_PHI, -PHI),
                Vector4::point(0.0, -INV_PHI,  PHI),
                Vector4::point(0.0, -INV_PHI, -PHI),
                Vector4::point( INV_PHI,  PHI, 0.0),
                Vector4::point( INV_PHI, -PHI, 0.0),
                Vector4::point(-INV_PHI,  PHI, 0.0),
                Vector4::point(-INV_PHI, -PHI, 0.0),
                Vector4::point( PHI, 0.0,  INV_PHI),
                Vector4::point( PHI, 0.0, -INV_PHI),
                Vector4::point(-PHI, 0.0,  INV_PHI),
                Vector4::point(-PHI, 0.0, -INV_PHI),
            ];

            let polygons = vec![
                Polygon::new(vec![points[0], points[8],  points[10], points[2],  points[16]],&spatial_props),
                Polygon::new(vec![points[0], points[16], points[17], points[1],  points[12]],&spatial_props),
                Polygon::new(vec![points[0], points[12], points[14], points[4],  points[8]],&spatial_props),
                Polygon::new(vec![points[3], points[11], points[9],  points[1],  points[17]],&spatial_props),
                Polygon::new(vec![points[3], points[17], points[16], points[2],  points[13]],&spatial_props),
                Polygon::new(vec![points[3], points[13], points[15], points[7],  points[11]],&spatial_props),
                Polygon::new(vec![points[5], points[14], points[12], points[1],  points[9]],&spatial_props),
                Polygon::new(vec![points[5], points[9],  points[11], points[7],  points[19]],&spatial_props),
                Polygon::new(vec![points[5], points[19], points[18], points[4],  points[14]],&spatial_props),
                Polygon::new(vec![points[6], points[15], points[13], points[2],  points[10]],&spatial_props),
                Polygon::new(vec![points[6], points[10], points[8],  points[4],  points[18]],&spatial_props),
                Polygon::new(vec![points[6], points[18], points[19], points[7],  points[15]],&spatial_props),
            ];

            let mesh_shape = MeshShape::new(spatial_props, lighting_props, polygons);
            Some(Self{mesh_shape})
            },
            None => None,
        }
    }

}
impl Shape for Dodecahedron{
    fn lighting_props(&self) -> &LightingProps{
        return &self.mesh_shape.lighting_props;
    }
    fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64, max: f64) -> Option<(f64,Vector4,Vector4)>{
        return self.mesh_shape.check_collision(origin, ray, min, max);
    }
}

pub struct Tetrahedron{
    pub mesh_shape: MeshShape,
}

impl Tetrahedron {
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Option<Self>{
        return match MeshShape::read_from_tokens(tokens){
                Some((spatial_props, lighting_props)) => {
                let half_height = f64::sqrt(2.0/3.0);
                let half_triangle_height = f64::sqrt(3.0) / 2.0;
                let base_left = Vector4::point(-1.0,-half_height,half_triangle_height);
                let base_right = Vector4::point(1.0,-half_height,half_triangle_height);
                let base_back = Vector4::point(0.0,-half_height,-half_triangle_height);
                let top = Vector4::point(0.0,half_height,0.0);

                let polygons=vec![
                    Polygon::new(vec![base_left, base_right, base_back], &spatial_props),
                    Polygon::new(vec![base_left, base_right, top], &spatial_props),
                    Polygon::new(vec![base_left, base_back, top], &spatial_props),
                    Polygon::new(vec![base_back, base_right, top], &spatial_props),
                ];

                let mesh_shape = MeshShape::new(spatial_props, lighting_props, polygons);
                Some(Self{mesh_shape})
            },
            None => None,
        }
    }
}
impl Shape for Tetrahedron{
    fn lighting_props(&self) -> &LightingProps{
        return &self.mesh_shape.lighting_props;
    }
    fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64, max: f64) -> Option<(f64,Vector4,Vector4)>{
        return self.mesh_shape.check_collision(origin, ray, min, max);
    }
}

impl fmt::Display for Tetrahedron{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let spatial_props = &self.mesh_shape.spatial_props;
        let lighting_props = &self.mesh_shape.lighting_props;
        write!(f, "Tetrahedron with scale {}, rotation X:{} Y:{} Z:{} and color {} located at {}.\n", spatial_props.scale, spatial_props.r_x, spatial_props.r_y, spatial_props.r_z, lighting_props.color, spatial_props.pos)?;
        return write!(f, "Lighting coefficients: amb:{} diff:{} spec:{} refl:{} bright:{}.", lighting_props.amb, lighting_props.diff, lighting_props.spec, lighting_props.refl, lighting_props.bright);
    }
}

pub struct Cube{
    pub mesh_shape: MeshShape,
}

impl Cube {
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Option<Self>{
        return match MeshShape::read_from_tokens(tokens){
            Some((spatial_props, lighting_props)) => {
                let polygons=vec![
                Polygon::new(vec![Vector4::point( 1.0,1.0,1.0),Vector4::point(1.0,1.0,-1.0),Vector4::point(1.0,-1.0,-1.0),Vector4::point( 1.0,-1.0,1.0)], &spatial_props),
                Polygon::new(vec![Vector4::point(-1.0,1.0,1.0),Vector4::point(-1.0,1.0,-1.0),Vector4::point(-1.0,-1.0,-1.0),Vector4::point(-1.0,-1.0,1.0)], &spatial_props),
                Polygon::new(vec![Vector4::point(-1.0,1.0,1.0),Vector4::point(1.0,1.0,1.0),Vector4::point(1.0,1.0,-1.0),Vector4::point(-1.0,1.0,-1.0)], &spatial_props),
                Polygon::new(vec![Vector4::point(-1.0,-1.0,1.0),Vector4::point(1.0,-1.0,1.0),Vector4::point(1.0,-1.0,-1.0),Vector4::point(-1.0,-1.0,-1.0)], &spatial_props),
                Polygon::new(vec![Vector4::point(-1.0,1.0,1.0),Vector4::point(1.0,1.0,1.0),Vector4::point(1.0,-1.0,1.0),Vector4::point(-1.0,-1.0,1.0)], &spatial_props),
                Polygon::new(vec![Vector4::point(-1.0,1.0,-1.0),Vector4::point(1.0,1.0,-1.0),Vector4::point(1.0,-1.0,-1.0),Vector4::point(-1.0,-1.0,-1.0)], &spatial_props)];

                let mesh_shape = MeshShape::new(spatial_props, lighting_props, polygons);
                Some(Self{mesh_shape})
            },
            None => None,
        }
    }
}
impl Shape for Cube{
    fn lighting_props(&self) -> &LightingProps{
        return &self.mesh_shape.lighting_props;
    }
    fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64, max: f64) -> Option<(f64,Vector4,Vector4)>{
        return self.mesh_shape.check_collision(origin, ray, min, max);
    }
}
impl fmt::Display for Cube{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let spatial_props = &self.mesh_shape.spatial_props;
        let lighting_props = &self.mesh_shape.lighting_props;
        write!(f, "Tetrahedron with scale {}, rotation X:{} Y:{} Z:{} and color {} located at {}.\n", spatial_props.scale, spatial_props.r_x, spatial_props.r_y, spatial_props.r_z, lighting_props.color, spatial_props.pos)?;
        return write!(f, "Lighting coefficients: amb:{} diff:{} spec:{} refl:{} bright:{}.", lighting_props.amb, lighting_props.diff, lighting_props.spec, lighting_props.refl, lighting_props.bright);
    }
}

pub struct Sphere{
    pub spatial_props: SpatialProps,
    pub lighting_props: LightingProps,
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
        let spatial_props = SpatialProps::new(pos, scale, r_x, r_y, r_z);

        let lighting_values = &parsed_tokens[lighting_param_start..];
        let color = Vector4::vec_from_slice(&parsed_tokens[color_start..lighting_param_start]);
        let amb = lighting_values[0];
        let diff = lighting_values[1];
        let spec = lighting_values[2];
        let refl = lighting_values[3];
        let bright = lighting_values[4];

        let lighting_props = LightingProps::new(color, amb, diff, spec, refl, bright);
        return Some(Self{spatial_props, lighting_props});
    }
}

impl Shape for Sphere{
    fn lighting_props(&self) -> &LightingProps{
        return &self.lighting_props;
    }
    fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64, max: f64) -> Option<(f64,Vector4,Vector4)>{
        let origin_prime = &self.spatial_props.inv_matrix * origin;
        let ray_prime = &self.spatial_props.inv_matrix * ray;
        
        let a = ray_prime.dot(&ray_prime);
        let b = origin_prime.dot(&ray_prime);
        let origin_len_sq = origin_prime.dot(&origin_prime);
        let c = origin_len_sq - 1.0;

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
    
                if a > origin_len_sq {
                    col_pt_prime *= -1.0;
                }
                col_pt_prime.force_vec();
                let mut normal =  &self.spatial_props.inv_transp * &col_pt_prime;
                normal.force_vec();

                return Some((t, col_pt, normal));
            }
        }
        return None;
    }
}

impl fmt::Display for Sphere{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere with scale {}, rotation X:{} Y:{} Z:{} and color {} located at {}.\n", self.spatial_props.scale, self.spatial_props.r_x, self.spatial_props.r_y, self.spatial_props.r_z, self.lighting_props.color, self.spatial_props.pos)?;
        return write!(f, "Lighting coefficients: amb:{} diff:{} spec:{} refl:{} bright:{}.", self.lighting_props.amb, self.lighting_props.diff, self.lighting_props.spec, self.lighting_props.refl, self.lighting_props.bright);
    }
}

