use std::path::PathBuf;

use clap::Parser;

use crate::{process::process_http_server, utils::verify_path, CmdExector};

#[derive(Parser, Debug)]
pub struct HttpOpts {
    #[arg(long, short, default_value = ".", value_parser = verify_path)]
    pub path: PathBuf,
    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExector for HttpOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        process_http_server(self.path.clone(), self.port).await
    }
}
