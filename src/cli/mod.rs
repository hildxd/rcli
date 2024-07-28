mod http_server;
mod jwt;

use clap::Parser;
use enum_dispatch::enum_dispatch;

pub use http_server::HttpOpts;
pub use jwt::{JwtSubCommand, SignClaimOpts, VerifyOpts};

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
    #[command(subcommand, about = "jwt encode/decode")]
    Jwt(JwtSubCommand),
}
