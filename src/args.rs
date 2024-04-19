use clap::Parser;

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
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    /// Delimiter, default is comma
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    /// CSV has header
    #[arg(short, long, action = clap::ArgAction::Set, default_value_t = true)]
    pub csv_header: bool,
}

fn valid_file_path(s: &str) -> Result<String, &'static str> {
    if std::path::Path::new(s).exists() {
        Ok(s.into())
    } else {
        Err("File not found")
    }
}
