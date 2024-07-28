use anyhow::Result;
use chrono::Duration;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    process::{process_decode_jwt, process_encode_jwt},
    CmdExector,
};

// 创建一个新类型包装 chrono::Duration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SerializableDuration(pub Duration);

impl SerializableDuration {
    pub fn new(duration: Duration) -> Self {
        SerializableDuration(duration)
    }

    pub fn into_inner(self) -> Duration {
        self.0
    }
}

// 为新类型实现 Serialize
impl Serialize for SerializableDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let nanos = self.0.num_nanoseconds().unwrap_or(i64::MAX);
        serializer.serialize_i64(nanos)
    }
}

impl<'de> Deserialize<'de> for SerializableDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let nanos = i64::deserialize(deserializer)?;
        Ok(SerializableDuration(Duration::nanoseconds(nanos)))
    }
}

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
    #[arg(long, short, value_parser = parse_duration)]
    pub exp: SerializableDuration,
    #[arg(long, default_value = "asdiuop123")]
    pub secret: String,
}

impl CmdExector for SignClaimOpts {
    async fn execute(&self) -> Result<()> {
        process_encode_jwt(self).await?;
        Ok(())
    }
}

fn parse_duration(s: &str) -> Result<SerializableDuration> {
    let len = s.len();
    let (num, unit) = s.split_at(len - 1);
    let num: usize = num.parse()?;
    match unit {
        "s" => Ok(SerializableDuration(Duration::seconds(num as i64))),
        "m" => Ok(SerializableDuration(Duration::minutes(num as i64))),
        "h" => Ok(SerializableDuration(Duration::hours(num as i64))),
        "d" => Ok(SerializableDuration(Duration::days(num as i64))),
        _ => anyhow::bail!("Invalid duration unit. Use d, h, m, or s."),
    }
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
        assert_eq!(
            parse_duration("1s").unwrap(),
            SerializableDuration(Duration::seconds(1))
        );
        assert_eq!(
            parse_duration("1m").unwrap(),
            SerializableDuration(Duration::minutes(1))
        );
        assert_eq!(
            parse_duration("1h").unwrap(),
            SerializableDuration(Duration::hours(1))
        );
        assert_eq!(
            parse_duration("1d").unwrap(),
            SerializableDuration(Duration::days(1))
        );
        assert_eq!(
            parse_duration("12d").unwrap(),
            SerializableDuration(Duration::days(12))
        );
        assert!(parse_duration("1x").is_err());
        assert!(parse_duration("100x").is_err());
    }
}
