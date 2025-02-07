use std::fmt;
use std::ops;
pub struct Matrix4{
    m: [f64; 16],
}

impl Matrix4{
    pub fn raw(m: [f64;16]) -> Self{
        return Self{m};
    }
    pub fn new() -> Self{
        return Self{m: [0.0;16]};
    }

    pub fn unit() -> Self{
        return Self::scale(1.0, 1.0, 1.0);
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Self{
        let mut matrix = Self::new();
        matrix.m[0] = x;
        matrix.m[5] = y;
        matrix.m[10] = z;
        matrix.m[15] = 1.0;
        return matrix;
    }

    pub fn trans(x: f64, y: f64, z: f64) -> Self{
        let mut matrix = Self::scale(1.0,1.0,1.0);
        matrix.m[3] = x;
        matrix.m[7] = y;
        matrix.m[11] = z;
        matrix.m[15] = 1.0;
        return matrix;
    }

    pub fn rot_x(angle: f64) -> Self{
        let cos = angle.cos();
        let sin = angle.sin();
        let mut matrix = Self::new();
        matrix.m[0] = 1.0;
        matrix.m[5] = cos;
        matrix.m[6] = -sin;
        matrix.m[9] = sin;
        matrix.m[10] = cos;
        matrix.m[15] = 1.0;
        return matrix;
    }
    pub fn rot_y(angle: f64) -> Self{
        let cos = angle.cos();
        let sin = angle.sin();
        let mut matrix = Self::new();
        matrix.m[0] = cos;
        matrix.m[2] = sin;
        matrix.m[5] = 1.0;
        matrix.m[8] = -sin;
        matrix.m[10] = cos;
        matrix.m[15] = 1.0;
        return matrix;
    }

    pub fn rot_z(angle: f64) -> Self{
        let cos = angle.cos();
        let sin = angle.sin();
        let mut matrix = Self::new();
        matrix.m[0] = cos;
        matrix.m[1] = - sin;
        matrix.m[4] = sin;
        matrix.m[5] = cos;
        matrix.m[10] = 1.0;
        matrix.m[15] = 1.0;
        return matrix;
    }

    pub fn transpose(&self) -> Self{
        let mut arr: [f64; 16] = [0.0; 16];
        for row in 0..4{
            for col in 0..4{
                arr[row * 4 + col] = self.m[row + col * 4];
            }
        }
        return Self::raw(arr);
    }

    pub fn inverse(&self) -> Self{
        let mut inverse = Self::scale(1.0, 1.0, 1.0);
        let mut clone = self.clone();

        for row in 0..4{
            let coeff = 1.0 / clone.m[row * 4 + row];
            for col in 0..4{
                clone.m[row * 4 + col] *= coeff;
                inverse.m[row * 4 + col] *= coeff;
            }

            for otherRow in 0..4{
                if otherRow == row{
                    continue
                }
                let coeff = clone.m[otherRow * 4 + row];
                for col in 0..4{
                    clone.m[otherRow * 4 + col] -= coeff * clone.m[row * 4 + col];
                    inverse.m[otherRow * 4 + col] -= coeff * inverse.m[row * 4 + col];
                }
            }
        }
        return inverse;
    }
}
impl Clone for Matrix4{
    fn clone(&self) -> Self{
        let m = self.m.clone();
        return Self{m};
    }
}

impl ops::Mul<&Matrix4> for &Matrix4{
    type Output = Matrix4;
    fn mul(self, rhs:&Matrix4) -> Matrix4{
        let mut prod = [0.0;16];
        for row in 0..4 {
            let row_start = row * 4;
            for col in 0..4{
                let mut sum = 0.0;
                for i in 0..4{
                    sum += self.m[row_start + i]  * rhs.m[col + 4*i];
                }
                prod[row * 4 + col] = sum;
            }
        }
        return Matrix4::raw(prod);
    }
}

impl ops::Mul<&Vector4> for &Matrix4{
    type Output = Vector4;
    fn mul(self, rhs: &Vector4) -> Vector4 {
        let mut vec = [0.0;4];
        for row in 0..4 {
            let row_start = row * 4;
            for col in 0..4{
                vec[row] += rhs.arr[col] * self.m[row_start + col];
            }
        }
        return Vector4::raw(vec);
    }
}

impl fmt::Display for Matrix4{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..=3 {
            let row_start = i*4;
            write!(f, "|{:6.3},{:6.3},{:6.3},{:6.3}|\n",self.m[row_start + 0],self.m[row_start + 1], self.m[row_start + 2], self.m[row_start + 3])?
        }
        return Ok(());
    }
}

pub struct Vector4{
    arr: [f64; 4],
}
impl Vector4{
    pub fn x(&self) -> f64{
        return self.arr[0];
    }
    pub fn y(&self) -> f64{
        return self.arr[1];
    }
    pub fn z(&self) -> f64{
        return self.arr[2];
    }
    pub fn force_vec(&mut self){
        self.arr[3] = 0.0;
    }
    pub fn force_point(&mut self){
        self.arr[3] = 1.0;
    }
    pub fn dot(&self, other: &Vector4) -> f64{
        return self.x() * other.x() + self.y() * other.y() + self.z() * other.z();
    }
    pub fn zero() -> Self{
        return Self{arr: [0.0; 4]};
    }
    pub fn raw(arr: [f64;4]) ->Self{
        return Self{arr};
    }
    pub fn new(x: f64, y: f64, z: f64, pt: f64) -> Self{
        return Self{arr:[x,y,z,pt]};
    }
    pub fn point(x: f64, y: f64, z:f64) -> Self{
        return Self{arr:[x,y,z,1.0]};
    }
    pub fn vec(x: f64, y: f64, z:f64) -> Self{
        return Self{arr:[x,y,z,0.0]};
    }
    pub fn to_rgb(&self) -> (u8, u8, u8){
        let mut red = self.arr[0];
        if red < 0.0{
            red = 0.0;
        }
        else if red > 1.0 {
            red = 1.0;
        }
        let mut green = self.arr[1];
        if green < 0.0{
            green = 0.0;
        }
        else if green > 1.0{
            green = 1.0;
        }

        let mut blue = self.arr[2];
        if blue < 0.0{
            blue = 0.0;
        }
        else if blue > 1.0{
            blue = 1.0;
        }
        let red = (255.0 * red) as u8;
        let green = (255.0 * green) as u8;
        let blue = (255.0 * blue) as u8;
        return (red, green, blue);
    }
    pub fn vec_from_slice(slice: &[&str]) -> Option<Self>{
        if slice.len() != 3 {
            return None;
        }
        let x_opt  = slice[0].to_string().trim().parse::<f64>();
        let y_opt = slice[1].to_string().trim().parse::<f64>();
        let z_opt  = slice[2].to_string().trim().parse::<f64>();
        if x_opt.is_err() || y_opt.is_err() || z_opt.is_err(){
            return None;
        }
        else{
            let x = x_opt.unwrap();
            let y = y_opt.unwrap();
            let z = z_opt.unwrap();
            return Some(Self::vec(x, y, z));
        }
    }
    pub fn point_from_slice(slice: &[&str]) -> Option<Self>{
        if slice.len() != 3 {
            return None;
        }
        let x_opt  = slice[0].to_string().trim().parse::<f64>();
        let y_opt = slice[1].to_string().trim().parse::<f64>();
        let z_opt  = slice[2].to_string().trim().parse::<f64>();
        if x_opt.is_err() || y_opt.is_err() || z_opt.is_err(){
            return None;
        }
        else{
            let x = x_opt.unwrap();
            let y = y_opt.unwrap();
            let z = z_opt.unwrap();
            return Some(Self::point(x, y, z));
        }
        
    }
}
impl Clone for Vector4{
    fn clone(&self) -> Self{
        let arr = self.arr.clone();
        return Self{arr};
    }
}

impl ops::Add<&Vector4> for &Vector4{
    type Output = Vector4;
    fn add(self, rhs: &Vector4) -> Vector4{
        let mut new: Vector4 = self.clone(); 
        for i in 0..4{
            new.arr[i] += rhs.arr[i];
        }
        return new;
    }
}

impl ops::MulAssign<f64> for Vector4{
    fn mul_assign(&mut self, rhs: f64){
        for i in 0..4{
            self.arr[i] *= rhs;
        }
    }
}
impl ops::SubAssign<&Vector4> for Vector4{
    fn sub_assign(&mut self, rhs: &Vector4){
        for i in 0..4{
            self.arr[i] -= rhs.arr[i];
        }
    }
}
impl ops::AddAssign<&Vector4> for Vector4{
    fn add_assign(&mut self, rhs: &Vector4){
        for i in 0..4{
            self.arr[i] += rhs.arr[i];
        }
    }
}
impl ops::MulAssign<&Vector4> for Vector4{
    fn mul_assign(&mut self, rhs: &Vector4){
        for i in 0..4{
            self.arr[i] *= rhs.arr[i];
        }
    }
}
impl fmt::Display for Vector4{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({:.3},{:.3},{:.3},{:.3})", self.arr[0], self.arr[1], self.arr[2], self.arr[3]);
    }
}