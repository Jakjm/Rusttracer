use std::fmt;
use crate::matrix::{Vector4,Matrix4};

pub struct SpatialProps {
    pub pos: Vector4,
    pub scale: Vector4,
    pub r_x: f64, 
    pub r_y: f64,
    pub r_z: f64,
    pub inv_matrix: Matrix4,
    pub inv_transp: Matrix4,
}
impl SpatialProps{
    pub fn new( pos: Vector4, scale: Vector4, r_x: f64, r_y: f64, r_z: f64 ) -> Self{
        let trans_matrix = Matrix4::trans(pos.x(),pos.y(),pos.z());
        let scale_matrix = Matrix4::scale(scale.x(),scale.y(),scale.z());
        let rot_x_matrix = Matrix4::rot_x(r_x);
        let rot_y_matrix = Matrix4::rot_y(r_y);
        let rot_z_matrix = Matrix4::rot_z(r_z);

        let rotation_matrix = &rot_z_matrix * &(&rot_y_matrix * &rot_x_matrix);
        let inv_matrix = (&trans_matrix * &(&rotation_matrix * &scale_matrix)).inverse();
        let inv_transp = inv_matrix.transpose();
        return Self{pos, scale, r_x, r_y, r_z, inv_matrix, inv_transp};
    }
}
//A convex planar polygon in 3D space, used to construct more complex 3 Dimensional shapes.
pub struct Polygon{
    points: Vec<Vector4>,
    inverse_axes: Vec<Vector4>,
    min_max_projections: Vec<(f64,f64)>,
    normal: Vector4,
    normal_prime: Vector4,
}

impl Polygon{
    pub fn new(mut points: Vec<Vector4>, spatial_props : &SpatialProps) -> Self{
        let mut inverse_axes = Vec::<Vector4>::with_capacity(points.len());
        let mut min_max_projections = Vec::<(f64,f64)>::with_capacity(points.len());
        
        let mut ab = points[1].clone();
        ab -= &points[0];
        let mut bc = points[2].clone();
        bc -= &points[1];

        let mut normal = ab.cross(&bc);
        let mut a = points[0].clone();
        a.force_vec();
        if normal.dot(&a) < 0.0 {
            normal *= -1.0;
        }

        let normal_prime = normal.apply_inv_transpose(&spatial_props.inv_transp); 
        for i in 0..points.len(){
            let mut vec = points[i];
            let prev_point = match i {
                0 => points[points.len() - 1],
                _ => points[i - 1],
            };
            vec -= &prev_point;
            
            let mut inv_axis = vec.cross(&normal);
            inv_axis.normalize();
            inverse_axes.push(inv_axis);

            let mut min = std::f64::INFINITY;
            let mut max = std::f64::NEG_INFINITY;
            for point in points.iter() {
                let dot = point.dot(&inv_axis);
                if dot > max
                {
                    max = dot;
                }
                if dot < min
                {
                    min = dot;
                }
            }
            min_max_projections.push((min, max));
        }
       
        return Self{points, inverse_axes, min_max_projections, normal, normal_prime};
    }
    pub fn check_collision(&self, origin: &Vector4, ray: &Vector4, min: f64, max:f64, inv_matrix: &Matrix4) -> Option<(f64,Vector4,Vector4)>{
        let origin_prime = inv_matrix * origin;
        let ray_prime = inv_matrix * ray;

        let ray_proj = ray_prime.dot(&self.normal);
        let origin_proj = origin_prime.dot(&self.normal);
        
        let surface_proj = self.points[0].dot(&self.normal);
        let distance = surface_proj - origin_proj;
        //let distance = 1.0 - origin_proj;
        let t = distance / ray_proj;
        if t <= min || t >= max{
            return None;
        }
            
        let mut col_pt_prime = ray_prime;
        col_pt_prime *= t;
        col_pt_prime += &origin_prime;

        for (inv_axis,(p_min, p_max)) in self.inverse_axes.iter().zip(self.min_max_projections.iter()){
            let dot = col_pt_prime.dot(inv_axis);
            if dot < *p_min || dot > *p_max {
                return None;
            }
        }

        let mut col_pt = ray.clone();
        col_pt *= t;
        col_pt += &origin;
        col_pt.force_point(); 
        return Some((t, col_pt, self.normal_prime));
    }
}
impl fmt::Display for Polygon{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Polygon:\n")?;
        write!(f, "Normal: {}\n", self.normal)?;
        for pt in self.points.iter(){
            write!(f, "{}\n", pt)?;
        }
        return write!(f, "\n");
    }
}
