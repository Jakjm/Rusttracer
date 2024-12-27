use std::fmt;
struct Vector{
    x: f64, 
    y: f64,
    z: f64,
}
impl Vector{
    fn new(x: f64, y: f64, z: f64) -> Self{
        Self{x,y,z}
    }
}
impl fmt::Display for Vector{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({},{},{})", self.x, self.y, self.z);
    }
}
struct Sphere{
    pos: Vector,

}
impl Sphere{
    fn new(pos: Vector) -> Self{
        Self{pos}
    }
}
impl fmt::Display for Sphere{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Sphere at {}", self.pos);
    }
}
fn main(){
    let pos = Vector::new(5.0,10.0,20.0);
    let sphere = Sphere::new(pos);
    println!("{}", sphere);
}
