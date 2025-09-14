use std::env;

mod matrix;
mod polygon;
mod shape;
use crate::matrix::Matrix4;
mod elements;
mod mesh_shape;
mod renderdata;
use crate::renderdata::RenderData;

const MAX_THREADS : usize = 255;
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
fn parse_command_line_options(args: &[String]) -> Option<(&String, u32, usize)>{    
    let mut file_name = None;
    let mut thread_count = None;
    let mut sample_amt = None;

    let mut iter = args.iter();
    while let Some(s) = iter.next(){
        if *s == "-a" && sample_amt == None {
            sample_amt = Some(8);
        }
        else if *s == "-t" && thread_count == None{
            let number = match iter.next(){
                Some(num) => num,
                None => {
                    println!("Please enter an integer thread count");
                    return None;
                },
            };
            let parse_result = number.to_string().trim().parse::<usize>();
            thread_count = match parse_result{
                Err(_e) => {
                    println!("Please enter an integer thread count");
                    return None;
                },
                Ok(num) => {
                    if num == 0 || num > MAX_THREADS {
                        println!("Please enter a positive thread count that is at most {MAX_THREADS}.");
                        return None;
                    }
                    Some(num)
                },
            };
        }
        else if file_name == None{
            file_name = Some(s);
        }
        else{
            println!("Could not understand command line arguments, please verify what you have entered!");
            return None;
        }
    }
    
    let file_name = match file_name{
        Some(filename) => filename,
        None => {
            return None;
        },
    };
    let thread_count = match thread_count{
        Some(thread_cnt) => thread_cnt,
        None => 1,
    };
    let sample_amt = match sample_amt{
        Some(sample_amt) => sample_amt,
        None => 0,
    };
    return Some((&file_name, sample_amt, thread_count));
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Insufficient arguments - please provide a path to an input file describing a scene.");
        return;
    }
    let (filename, sample_count, thread_count) = match parse_command_line_options(&args[1..]){
        Some((filename, sample_count, thread_count)) => (filename, sample_count, thread_count),
        None => {
            println!("Could not recognize command line arguments!");
            return;
        }
    };
    match RenderData::read_from_file(filename){
        Ok(file_data) => {
            println!("{}", file_data);
            //let capacity = (file_data.width * file_data.height) as usize;
            //let mut array = vec![self.back_color.clone(); capacity];
            let array = file_data.render(sample_count, thread_count);
            let _ = file_data.save_image(array);
        },
        Err(error) => println!("{error}"),
    }
}
