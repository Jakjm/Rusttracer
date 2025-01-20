use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod matrix;
use crate::matrix::Vector4;
use crate::matrix::Matrix4;
mod elements;
use crate::elements::Sphere;

mod renderdata;
use crate::renderdata::RenderData;


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
        let data = RenderData::read_from_file(&args[1]);
        if let Ok(file_data) = &data {
            println!("{}", file_data);
        }
        else if let Err(error) = &data{
            println!("{error}");
        }
        
    }
    Ok(())
}
