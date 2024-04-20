use anyhow::Result;
use clap::Parser;
use rcli::{csv_to_json, generate_password, Args, Command};

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);
    match args.cmd {
        Command::Csv(opts) => {
            csv_to_json(&opts.input, &opts.output, opts.format)?;
        }
        Command::GenPass(opts) => {
            generate_password(
                opts.length,
                opts.lowercase,
                opts.uppercase,
                opts.numbers,
                opts.special,
            )?;
        }
    }
    Ok(())
}
