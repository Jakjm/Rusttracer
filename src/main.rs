use std::env;

mod matrix;
use crate::matrix::Matrix4;
mod elements;

mod renderdata;
use crate::renderdata::RenderData;


fn test(){

    let matrix = Matrix4::scale(1.0,2.0,3.0);
    println!("{}",matrix);

    let matrix2 = Matrix4::trans(5.0,0.0,5.0);
    println!("{}",matrix2);

    let product = &matrix * &matrix2;
    println!("Product:\n{}", product);

    let inverse = product.inverse();
    println!("Inverse:\n{}", inverse);

    let trans = inverse.transpose();
    println!("Transpose:\n{}", trans);
}
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Input file not found!");
    }
    else if args[1] == "TEST"{
        test();
    }
    else{
        let file_name_index: usize;
        let sample_amt: u32;
        if args[1] == "-a" {
            file_name_index = 2;
            sample_amt = 4;
        }
        else{
            file_name_index = 1;
            sample_amt = 0;
        }

        let mut data = RenderData::read_from_file(&args[file_name_index]);
        if let Ok(file_data) = &mut data {
            println!("{}", file_data);
            file_data.render(sample_amt);
            file_data.save_image();
        }
        else if let Err(error) = &data{
            println!("{error}");
        }
    }
    Ok(())
}
