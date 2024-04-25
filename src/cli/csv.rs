use crate::process::valid_file;
use crate::{csv_to_json, CmdExecutor};
use clap::Parser;
use std::fmt::{Display, Formatter};

/// CSV options
#[derive(Debug, Parser)]
#[clap(name = "CSV Options", author, version, about = "Convert CSV to JSON")]
pub struct CsvOpts {
    /// Input CSV file
    #[arg(short, long, value_parser = valid_file)]
    pub input: String,
    /// Output file name, default is output
    #[arg(short, long, default_value = "output")]
    pub output: String,
    /// Output format
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

impl From<OutputFormat> for String {
    fn from(format: OutputFormat) -> Self {
        format.to_string()
    }
}

impl CmdExecutor for CsvOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        csv_to_json(&self.input, &self.output, self.format)?;
        Ok(())
    }
}
