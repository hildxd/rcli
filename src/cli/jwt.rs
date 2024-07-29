use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::{
    process::{process_decode_jwt, process_encode_jwt},
    CmdExector,
};

// 创建一个新类型包装 chrono::Duration

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "sign a claim")]
    Sign(SignClaimOpts),
    #[command(name = "verify", about = "verify a claim")]
    Verify(VerifyOpts),
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct SignClaimOpts {
    #[arg(long, short)]
    pub sub: String,
    #[arg(long, short)]
    pub aud: String,
    #[arg(long, short, value_parser = parse_exp)]
    pub exp: usize,
    #[arg(long, default_value = "asdiuop123")]
    pub secret: String,
}

impl CmdExector for SignClaimOpts {
    async fn execute(&self) -> Result<()> {
        process_encode_jwt(self).await?;
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

fn parse_exp(s: &str) -> Result<usize> {
    let duration = parse_duration(s)?;
    let expiration = Utc::now()
        .checked_add_signed(duration)
        .context("Invilid duration")?
        .timestamp();
    Ok(expiration as usize)
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(long, short)]
    pub token: String,
    #[arg(long, default_value = "asdiuop123")]
    pub secret: String,
}

impl CmdExector for VerifyOpts {
    async fn execute(&self) -> Result<()> {
        process_decode_jwt(self).await?;
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
        assert_eq!(parse_duration("12d").unwrap(), Duration::days(12));
        assert!(parse_duration("1x").is_err());
        assert!(parse_duration("100x").is_err());
    }
}
