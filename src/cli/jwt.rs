use anyhow::Result;
use chrono::Duration;
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::CmdExector;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "sign a claim")]
    Sign(SignClaimOpts),
    #[command(name = "verify", about = "verify a claim")]
    Verify(VerifyOpts),
}

#[derive(Debug, Parser)]
pub struct SignClaimOpts {
    #[arg(long, short)]
    pub sub: String,
    #[arg(long, short)]
    pub company: String,
    #[arg(long, short, value_parser = parse_duration)]
    pub exp: Duration,
}

impl CmdExector for SignClaimOpts {
    async fn execute(&self) -> Result<()> {
        println!("{:?}", self);
        Ok(())
    }
}

fn parse_duration(s: &str) -> Result<Duration> {
    let len = s.len();
    let (num, unit) = s.split_at(len - 1);
    let num: usize = num.parse()?;
    match unit {
        "s" => Ok(Duration::seconds(num as i64)),
        "m" => Ok(Duration::minutes(num as i64)),
        "h" => Ok(Duration::hours(num as i64)),
        "d" => Ok(Duration::days(num as i64)),
        _ => anyhow::bail!("Invalid duration unit. Use d, h, m, or s."),
    }
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(long, short)]
    pub token: String,
}

impl CmdExector for VerifyOpts {
    async fn execute(&self) -> Result<()> {
        println!("{:?}", self);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("1s").unwrap(), Duration::seconds(1));
        assert_eq!(parse_duration("1m").unwrap(), Duration::minutes(1));
        assert_eq!(parse_duration("1h").unwrap(), Duration::hours(1));
        assert_eq!(parse_duration("1d").unwrap(), Duration::days(1));
        assert!(parse_duration("1x").is_err());
    }
}
