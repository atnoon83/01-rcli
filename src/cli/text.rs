use crate::process::{parse_text_sign_format, valid_file, verify_path};
use crate::{
    get_content, get_reader, process_text_key_generate, process_text_sign, process_text_verify,
    CmdExecutor,
};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubcommand {
    Sign(TextSignOpts),
    Verify(TextVerifyOpts),
    Generate(KeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, default_value = "-", value_parser = valid_file)]
    pub input: String,
    #[arg(short, long, value_parser = valid_file)]
    pub key: String,
    #[arg(short, long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, default_value = "-", value_parser = valid_file)]
    pub input: String,
    #[arg(short, long, value_parser = valid_file)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(short, long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Copy, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl CmdExecutor for TextSignOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let sig = process_text_sign(&mut reader, &key, self.format)?;
        let encoded = URL_SAFE_NO_PAD.encode(sig);
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let decoded = URL_SAFE_NO_PAD.decode(&self.sig)?;
        let verified = process_text_verify(&mut reader, &key, &decoded, self.format)?;
        if verified {
            println!("✓ Signature verified");
        } else {
            println!("⚠ Signature not verified");
        }
        Ok(())
    }
}

impl CmdExecutor for KeyGenerateOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let key = process_text_key_generate(self.format)?;
        for (k, v) in key {
            fs::write(self.output_path.join(k), v)?;
        }
        Ok(())
    }
}
