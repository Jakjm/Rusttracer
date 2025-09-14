use crate::matrix::Vector4;
//use crate::matrix::Matrix4;

pub struct LightingProps {
    pub color: Vector4,
    pub amb: f64,
    pub diff: f64,
    pub spec: f64,
    pub refl: f64,
    pub bright: f64,
}

impl LightingProps {
    pub fn new( color: Vector4, amb: f64, diff: f64, spec: f64, refl: f64, bright: f64) -> Self{
        return Self{color, amb, diff, spec, refl, bright};
    }
}

pub trait Shape{
    fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64, max:f64) -> Option<(f64,Vector4,Vector4)>;
    fn lighting_props(&self) -> &LightingProps;
}