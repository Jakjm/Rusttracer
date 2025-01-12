use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod matrix;
use crate::matrix::Vector4;
use crate::matrix::Matrix4;
mod shape;
use crate::shape::Sphere;

fn test(){
    let pt = Vector4::point(3.0,5.0,5.0);
    println!("{}", pt);

    let matrix = Matrix4::scale(1.0,2.0,3.0);
    println!("{}",matrix);

    let matrix2 = Matrix4::trans(5.0,0.0,5.0);
    println!("{}",matrix2);

    println!("{}", &matrix * &pt);
    println!("{}", &matrix2 * &pt);
}
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Input file not found!");
    }
    else{
        let path = Path::new(&args[1]);
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let lines = (&mut reader).lines();
        for line in lines.map_while(Result::ok){
            println!("{}", Sphere::readFromString(line));
        }
    }
    Ok(())
}
