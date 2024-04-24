use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use clap::Parser;
use rcli::{
    base64_decode, base64_encode, csv_to_json, generate_password, get_content, get_reader,
    process_http_serve, process_text_key_generate, process_text_sign, process_text_verify, Args,
    Base64Subcommand, Command, TextSubcommand,
};
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    match args.cmd {
        Command::Csv(opts) => {
            csv_to_json(&opts.input, &opts.output, opts.format)?;
        }
        Command::GenPass(opts) => {
            let password = generate_password(
                opts.length,
                opts.lowercase,
                opts.uppercase,
                opts.numbers,
                opts.special,
            )?;
            println!("{}", password);
            let score = zxcvbn::zxcvbn(password.as_str(), &[])?.score();
            eprintln!("Password strength: {}", score);
        }
        Command::Base64(cmd) => match cmd {
            Base64Subcommand::Encode(opts) => {
                println!("{}", base64_encode(&opts.input, opts.format)?);
            }
            Base64Subcommand::Decode(opts) => {
                println!("{}", base64_decode(&opts.input, opts.format)?);
            }
        },
        Command::Text(cmd) => match cmd {
            TextSubcommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            TextSubcommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
            TextSubcommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
        },
        Command::Serve(opts) => {
            process_http_serve(opts.dir, opts.port).await?;
        }
    }
    Ok(())
}
