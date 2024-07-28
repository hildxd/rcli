mod http_server;
use clap::Parser;

use enum_dispatch::enum_dispatch;
pub use http_server::HttpOpts;

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "http", about = "start a http file server")]
    Http(HttpOpts),
}
