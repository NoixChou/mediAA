use std::fs;
use std::io;
use std::io::Write;
use std::path;

#[derive(Debug, thiserror::Error)]
pub enum AsciiError {
    #[error("Failed to open output file")]
    Open(#[source] io::Error),
    #[error("Failed to write to output file")]
    Write(#[source] io::Error),
}

pub struct AsciiText(Vec<String>);

impl AsciiText {
    pub fn new(lines: Vec<String>) -> Self {
        Self(lines)
    }
    
    pub fn print(&self) {
        for line in &self.0 {
            println!("{}", line);
        }
    }
    
    pub fn write_to_file(&self, dest_file_path: String) -> Result<(), AsciiError> {
        let mut dest_file = fs::File::create(dest_file_path).map_err(|e| AsciiError::Open(e))?;
        
        for line in &self.0 {
            writeln!(dest_file, "{}", line).map_err(|e| AsciiError::Write(e))?;
        }
        dest_file.flush().map_err(|e| AsciiError::Write(e))?;
        
        Ok(())
    }
}