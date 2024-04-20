use anyhow::Result;
use clap::Parser;
use rcli::{
    base64_decode, base64_encode, csv_to_json, generate_password, Args, Base64Subcommand, Command,
};

fn main() -> Result<()> {
    let args = Args::parse();
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
        Command::Base64(cmd) => match cmd {
            Base64Subcommand::Encode(opts) => {
                base64_encode(&opts.input, opts.format)?;
            }
            Base64Subcommand::Decode(opts) => {
                base64_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
