use crate::ascii_text::{AsciiText, AsciiError};

const ASCII_COLORS: &str = r##"WM$@%&#NB8E9GAmK6w5HRkbYT43V0JL7gpaesyxznocv?Ijftr1li|*=-~^`':;,. "##;

#[derive(Debug, thiserror::Error)]
pub enum BitmapError {
    #[error("Failed to convert media")]
    Convert(#[source] anyhow::Error),
}

pub struct Graymap {
    pub pixels: Vec<Vec<u8>>,
    pub width: u32,
    pub height: u32,
    pub output_scale: f64,
    pub is_invert_color: bool,
}

impl Graymap {
    pub fn to_text(&self) -> AsciiText {
        let out_width = (self.width as f64 * self.output_scale).floor() as usize;
        let out_height = (self.height as f64 * self.output_scale).floor() as usize;
        
        let mut ascii_text = Vec::new();
        
        for y in 0..out_height {
            let mut line = String::new();
            for x in 0..out_width {
                let mut ascii_index = ((self.pixels[(y as f64 / self.output_scale) as usize][(x as f64 / self.output_scale) as usize] as f64 / 256.0) * ASCII_COLORS.len() as f64) as usize;
                if self.is_invert_color {
                    ascii_index = ASCII_COLORS.len() - 1 - ascii_index;
                }
                let c = ASCII_COLORS.chars().nth(ascii_index).unwrap_or_else(|| panic!("index = {}", ascii_index));
                line.push(c);
                line.push(c);
            }
            ascii_text.push(line);
        }
        
        AsciiText::new(ascii_text)
    }
}