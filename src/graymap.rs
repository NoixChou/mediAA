const ASCII_COLORS: &str = r##"WM$@%&#NB8E9GAmKwHDF56R0Sgpabesdhftvxzi?jIl|!*+=~-^"'`;:,."##;

pub struct Graymap {
    pub pixels: Vec<Vec<u8>>,
    pub width: u32,
    pub height: u32,
    pub output_scale: f64
}

impl Graymap {
    pub fn to_text(&self) -> Result<Vec<String>, ()> {
        let out_width = (self.width as f64 * self.output_scale).floor() as usize;
        let out_height = (self.height as f64 * self.output_scale).floor() as usize;
        
        let mut ascii_text = Vec::new();
        
        for y in 0..out_height {
            let mut line = String::new();
            for x in 0..out_width {
                let c = ASCII_COLORS.chars().nth(((self.pixels[(y as f64 / self.output_scale) as usize][(x as f64 / self.output_scale) as usize] as f64 / 256.0) * ASCII_COLORS.len() as f64) as usize).unwrap();
                line.push(c);
                line.push(c);
            }
            ascii_text.push(line);
        }
        
        Ok(ascii_text)
    }
}