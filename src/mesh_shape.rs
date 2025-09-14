use crate::shape::{LightingProps,Shape};
use crate::polygon::{SpatialProps,Polygon};
//use std::fmt;
use crate::matrix::Vector4;

pub struct MeshShape {
    pub spatial_props: SpatialProps,
    pub lighting_props: LightingProps,
    pub polygons: Vec<Polygon>,
}

impl MeshShape {
    pub fn new(spatial_props: SpatialProps, lighting_props: LightingProps, polygons: Vec<Polygon>) -> Self{
        return Self{spatial_props, lighting_props, polygons};
    }
    pub fn read_from_tokens(tokens: &Vec<&str>) -> Option<(SpatialProps,LightingProps)>{
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

        let color = Vector4::vec_from_slice(&parsed_tokens[color_start..lighting_param_start]);
        let lighting_values = &parsed_tokens[lighting_param_start..];
        let amb = lighting_values[0];
        let diff = lighting_values[1];
        let spec = lighting_values[2];
        let refl = lighting_values[3];
        let bright = lighting_values[4];

        let lighting_props = LightingProps::new(color, amb, diff, spec, refl, bright);
        return Some((spatial_props, lighting_props));
    }
}
impl Shape for MeshShape{
    fn lighting_props(&self) -> &LightingProps{
        return &self.lighting_props;
    }
    fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64, max: f64) -> Option<(f64,Vector4,Vector4)>{
        let mut min_t = max;
        let mut col_data: Option<(f64,Vector4,Vector4)> = None; 
        for polygon in self.polygons.iter(){
            match polygon.check_collision(origin, ray, min, min_t, &self.spatial_props.inv_matrix) {
                Some((t,col_pt,normal)) => {
                    min_t = t;
                    col_data = Some((t, col_pt, normal));
                }
                None => {}
            }
        }
        return col_data;
    }
}