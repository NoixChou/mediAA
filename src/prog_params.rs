use std::env;
use std::ops::Add;

pub struct MediAAConfig {
    pub(crate) source_file_path: String,
    pub(crate) destination_file_path: Option<String>,
    pub(crate) output_scale: Option<f64>,
    pub(crate) is_invert_color: bool,
}

impl MediAAConfig {
    pub(crate) fn create(args: env::Args) -> Result<Self, ()> {
        let args: Vec<String> = args.collect();
        
        let source_file_path = args.get(1).ok_or(())?.clone();
        let mut destination_file_path = Some(source_file_path.clone().add(".txt"));
        let mut output_scale = Option::<f64>::None;
        let mut is_invert_color = false;
        
        for (i, arg) in args[2..].iter().enumerate() {
            match &arg[0..2] {
                "-o" => destination_file_path = Some(args.get(i + 3).ok_or(())?.clone()),
                "-s" => output_scale = args.get(i + 3).ok_or(())?.parse().ok(),
                "-v" => destination_file_path = None,
                "-i" => is_invert_color = true,
                _ => ()
            }
        }
        
        Ok(MediAAConfig {
            source_file_path,
            destination_file_path,
            output_scale,
            is_invert_color,
        })
    }
}
