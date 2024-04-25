use crate::{Base64Subcommand, CsvOpts, GenPassOpts, ServeOpts, TextSubcommand};
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[clap(
    name = "rcli",
    version,
    author = "abc",
    about = "Simple CLI command to convert from one format to another"
)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum Command {
    #[command(about = "Convert CSV to other format")]
    Csv(CsvOpts),
    #[command(about = "Generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64Subcommand),
    #[command(subcommand, about = "Text sign and verify")]
    Text(TextSubcommand),
    #[command(about = "Serve files over HTTP")]
    Serve(ServeOpts),
}

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(&self) -> anyhow::Result<()>;
}
