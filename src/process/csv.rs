use crate::process::valid_file_path;
use anyhow::Result;
use clap::Parser;
use csv::Reader;
use serde_json::Value;
use std::fmt::{Display, Formatter};

/// CSV options
#[derive(Debug, Parser)]
#[clap(name = "CSV Options", author, version, about = "Convert CSV to JSON")]
pub struct CsvOpts {
    /// Input CSV file
    #[arg(short, long, value_parser = valid_file_path)]
    pub input: String,
    /// Output JSON file, optionalq
    #[arg(short, long, default_value = "output")]
    pub output: String,
    #[arg(short, long)]
    pub format: OutputFormat,
    /// Delimiter, default is comma
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    /// CSV has header
    #[arg(short, long, action = clap::ArgAction::Set, default_value_t = true)]
    pub csv_header: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl std::str::FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "yaml" => Ok(Self::Yaml),
            _ => Err("Invalid output format"),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Json => "json",
                Self::Yaml => "yaml",
            }
        )
    }
}

/// Convert CSV to JSON
/// input: &str - input CSV file
/// output: &str - output JSON file
pub fn csv_to_json(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut vec = Vec::with_capacity(256);
    let headers = reader.headers()?.clone();
    for record in reader.records() {
        let record = record?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        vec.push(json_value);
    }

    let result = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&vec)?,
        OutputFormat::Yaml => serde_yaml::to_string(&vec)?,
    };

    let output = format!("{}.{}", output, format);

    std::fs::write(output, result)?;
    Ok(())
}
