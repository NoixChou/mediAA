mod image_to_bitmap;
mod graymap;
mod prog_params;
mod ascii_text;

use std::env;

fn main() -> anyhow::Result<()> {
    let configuration = prog_params::MediAAConfig::create(env::args()).unwrap_or_else(|_| exit_with_command_usage());
    
    let lines = image_to_bitmap::image_to_graymap(configuration.source_file_path, configuration.output_scale, configuration.is_invert_color)?.to_text();
    
    match configuration.destination_file_path {
        None => {
            lines.print();
            Ok(())
        }
        Some(dest_file_path) => {
            lines.write_to_file(dest_file_path)?;
            Ok(())
        }
    }
}

fn exit_with_command_usage() -> ! {
    print!("{}", command_usage());
    std::process::exit(0);
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