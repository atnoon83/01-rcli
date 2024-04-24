mod base64;
mod csv;
mod genpass;
mod serve;
mod test;

pub use base64::*;
pub use csv::*;
pub use genpass::*;
pub use serve::*;
use std::path::{Path, PathBuf};
pub use test::*;

pub fn valid_file(s: &str) -> std::result::Result<String, &'static str> {
    if s == "-" || std::path::Path::new(s).exists() {
        Ok(s.into())
    } else {
        Err("File not found")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

pub fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn std::io::Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(std::fs::File::open(input)?))
    }
}

pub fn get_content(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
