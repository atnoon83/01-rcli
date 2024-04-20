use clap::Parser;
use std::fmt::{Display, Formatter};

/// Simple CLI command to convert from one format to another
#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Command,
}

/// Subcommands
/// Csv - Convert CSV to JSON
#[derive(Debug, Parser)]
pub enum Command {
    Csv(CsvOpts),
}

/// CSV options
#[derive(Debug, Parser)]
#[clap(name = "rcli", author, version, about = "Convert CSV to JSON")]
pub struct CsvOpts {
    /// Input CSV file
    #[arg(short, long, value_parser = valid_file_path)]
    pub input: String,
    /// Output JSON file, optional
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

fn valid_file_path(s: &str) -> Result<String, &'static str> {
    if std::path::Path::new(s).exists() {
        Ok(s.into())
    } else {
        Err("File not found")
    }
}
