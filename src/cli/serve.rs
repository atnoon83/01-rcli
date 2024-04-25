use crate::{process_http_serve, CmdExecutor};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct ServeOpts {
    #[arg(short, long, default_value = "9527")]
    pub port: u16,
    #[arg(short, long, default_value = ".")]
    pub dir: PathBuf,
}

pub struct ServeState {
    pub(crate) dir: PathBuf,
}

impl CmdExecutor for ServeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        process_http_serve(self.dir.clone(), self.port).await?;
        Ok(())
    }
}
