use crate::process::valid_file;
use crate::{base64_decode, base64_encode, CmdExecutor};
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64Subcommand {
    Encode(EncodeOpts),
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
#[clap(name = "base64 encode", author, version, about = "Base64 encode")]
pub struct EncodeOpts {
    #[arg(short, long, default_value = "-", value_parser = valid_file)]
    pub input: String,
    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
#[clap(name = "base64 decode", author, version, about = "Base64 decode")]
pub struct DecodeOpts {
    #[arg(short, long, default_value = "-", value_parser = valid_file)]
    pub input: String,
    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Copy, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl CmdExecutor for EncodeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        println!("{}", base64_encode(&self.input, self.format)?);
        Ok(())
    }
}

impl CmdExecutor for DecodeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        println!("{}", base64_decode(&self.input, self.format)?);
        Ok(())
    }
}
