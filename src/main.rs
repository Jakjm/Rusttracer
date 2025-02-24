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
        //TODO parse more arguments!
        let sample_amt: u32;
        if args[1] == "-a" {
            file_name_index = 2;
            sample_amt = 8;
        }
        else{
            file_name_index = 1;
            sample_amt = 0;
        }

        match RenderData::read_from_file(&args[file_name_index]){
            Ok(file_data) => {
                println!("{}", file_data);
                //let capacity = (file_data.width * file_data.height) as usize;
                //let mut array = vec![self.back_color.clone(); capacity];
                let arrays = file_data.render(sample_amt, 4);
                file_data.save_image(&arrays);
            },
            Err(error) => println!("{error}"),
        }
    }
    Ok(())
}
