use anyhow::Result;
use clap::Parser;
use rcli::{csv_to_json, Args, Command};

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);
    match args.cmd {
        Command::Csv(opts) => {
            csv_to_json(&opts.input, &opts.output)?;
        }
    }

    Ok(())
}
