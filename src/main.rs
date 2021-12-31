mod image_to_bitmap;
mod graymap;

use std::{env, fs};
use std::io::Write;
use std::ops::Add;

const ERR_OPEN_OUTPUT: &str = "Failed to open output file";
const ERR_WRITE_OUTPUT: &str = "Failed to write to output file";
const ERR_CONVERT: &str = "Failed to convert file";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let src_file_path = match args.get(1) {
        None => {
            println!("{}", command_usage());
            return;
        }
        Some(p) => p.clone()
    };
    let mut dest_file_path = Some(src_file_path.clone().add(".txt"));
    let mut output_scale = Option::<f64>::None;
    let mut is_invert_color = false;
    
    for (i, arg) in args[2..].iter().enumerate() {
        match &arg[0..2] {
            "-o" => dest_file_path = Some(args.get(i + 3).expect(command_usage()).clone()),
            "-s" => output_scale = args.get(i + 3).expect(command_usage()).clone().parse().ok(),
            "-v" => dest_file_path = None,
            "-i" => is_invert_color = true,
            _ => ()
        }
    }
    
    match dest_file_path {
        None => {
            let lines = image_to_bitmap::image_to_graymap(src_file_path, output_scale, is_invert_color).expect(ERR_CONVERT).to_text().expect(ERR_CONVERT);
            
            for line in lines {
                println!("{}", line);
            }
        }
        Some(dest_file_path) => {
            let lines = image_to_bitmap::image_to_graymap(src_file_path, output_scale, is_invert_color).expect(ERR_CONVERT).to_text().expect(ERR_CONVERT);
            
            let mut dest_file = fs::File::create(dest_file_path).expect(ERR_OPEN_OUTPUT);
            
            for line in lines {
                writeln!(dest_file, "{}", line).expect(ERR_WRITE_OUTPUT);
            }
            dest_file.flush().expect(ERR_WRITE_OUTPUT);
            return;
        }
    }
}

fn command_usage() -> &'static str {
    "\
    usage: mediaa (source_file)
              [-o output_file]
              [-s output_scale]
              [-v (output to stdout)]
              [-i (invert color)]\
    "
}