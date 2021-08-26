mod image_to_bitmap;
mod graymap;
mod prog_params;

use std::{env, fs};
use std::io::Write;

const ERR_OPEN_OUTPUT: &str = "Failed to open output file";
const ERR_WRITE_OUTPUT: &str = "Failed to write to output file";
const ERR_CONVERT: &str = "Failed to convert file";

fn main() {
    let configuration = prog_params::MediAAConfig::create(env::args()).expect(command_usage());
    
    match configuration.destination_file_path {
        None => {
            let lines = image_to_bitmap::image_to_graymap(configuration.source_file_path, configuration.output_scale, configuration.is_invert_color).expect(ERR_CONVERT).to_text().expect(ERR_CONVERT);
            
            for line in lines {
                println!("{}", line);
            }
        }
        Some(dest_file_path) => {
            let lines = image_to_bitmap::image_to_graymap(configuration.source_file_path, configuration.output_scale, configuration.is_invert_color).expect(ERR_CONVERT).to_text().expect(ERR_CONVERT);
            
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
    usage: madiaa (source_file)
              [-o output_file]
              [-s output_scale]
              [-v (output to stdout)]
              [-i (invert color)]\
    "
}