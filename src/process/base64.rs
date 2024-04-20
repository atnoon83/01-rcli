use crate::process::valid_file_path;
use anyhow::Result;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use clap::Parser;
use std::io::Read;

#[derive(Debug, Parser)]
pub enum Base64Subcommand {
    Encode(EncodeOpts),
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
#[clap(name = "base64 encode", author, version, about = "Base64 encode")]
pub struct EncodeOpts {
    #[arg(short, long, default_value = "-", value_parser = valid_file_path)]
    pub input: String,
    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
#[clap(name = "base64 decode", author, version, about = "Base64 decode")]
pub struct DecodeOpts {
    #[arg(short, long, default_value = "-", value_parser = valid_file_path)]
    pub input: String,
    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Copy, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl std::str::FromStr for Base64Format {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Self::Standard),
            "urlsafe" => Ok(Self::UrlSafe),
            _ => Err("Invalid base64 format"),
        }
    }
}

impl std::fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Standard => "standard",
                Self::UrlSafe => "urlsafe",
            }
        )
    }
}

fn get_reader(input: &str) -> Result<Box<dyn std::io::Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(std::fs::File::open(input)?))
    }
}

pub fn base64_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    match format {
        Base64Format::Standard => {
            println!("{}", STANDARD.encode(&buf));
        }
        Base64Format::UrlSafe => {
            println!("{}", URL_SAFE_NO_PAD.encode(&buf));
        }
    }
    Ok(())
}

pub fn base64_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    match format {
        Base64Format::Standard => {
            println!("{}", String::from_utf8(STANDARD.decode(buf)?)?);
        }
        Base64Format::UrlSafe => {
            println!("{}", String::from_utf8(URL_SAFE_NO_PAD.decode(buf)?)?);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(base64_encode(input, format).is_ok());
        let format = Base64Format::UrlSafe;
        assert!(base64_encode(input, format).is_ok());
    }

    #[test]
    fn test_base64_decode() {
        let input = "output.b64";
        let format = Base64Format::Standard;
        assert!(base64_decode(input, format).is_ok());
        let format = Base64Format::UrlSafe;
        assert!(base64_decode(input, format).is_ok());
    }
}