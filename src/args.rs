use crate::{CsvOpts, GenPassOpts};
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
    GenPass(GenPassOpts),
}
