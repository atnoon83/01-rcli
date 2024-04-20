mod base64;
mod csv;
mod genpass;

pub use base64::*;
pub use csv::*;
pub use genpass::*;

pub fn valid_file_path(s: &str) -> std::result::Result<String, &'static str> {
    if s == "-" || std::path::Path::new(s).exists() {
        Ok(s.into())
    } else {
        Err("File not found")
    }
}
