use crate::{SignClaimOpts, VerifyOpts};
use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use tracing::info;

pub async fn process_encode_jwt(claims: &SignClaimOpts) -> Result<String> {
    info!("encode secert {:?}", claims.secret);
    let secret: &[u8] = claims.secret.as_bytes();
    let token = encode::<SignClaimOpts>(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(secret),
    )?;
    println!("{:?}", token);
    Ok(token)
}

pub async fn process_decode_jwt(opts: &VerifyOpts) -> Result<SignClaimOpts> {
    info!("decode secert {:?}", opts.secret);
    let token = decode::<SignClaimOpts>(
        &opts.token,
        &DecodingKey::from_secret(opts.secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    println!("{:?}", token.claims);
    Ok(token.claims)
}

#[cfg(test)]
mod test {
    use std::sync::RwLock;

    use chrono::Duration;

    use crate::SerializableDuration;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref MUTABLE_TOKEN: RwLock<String> = RwLock::new(String::new());
    }

    use super::*;
    const SECRET: &str = "asdiuop123";
    #[tokio::test]
    async fn test_process_encode_jwt() {
        let claims = SignClaimOpts {
            exp: SerializableDuration(Duration::seconds(1)),
            secret: SECRET.to_string(),
            sub: "test".to_string(),
            aud: "test".to_string(),
        };

        let token = process_encode_jwt(&claims).await;
        match token {
            Ok(token) => {
                let mut write = MUTABLE_TOKEN.write().unwrap();
                *write = token.clone();
                assert_eq!(token, *write)
            }
            Err(e) => {
                panic!("error {:?}", e);
            }
        };
    }

    #[tokio::test]
    async fn test_process_decode_jwt() {
        let token = MUTABLE_TOKEN.read().unwrap();
        let res = process_decode_jwt(&VerifyOpts {
            token: token.clone(),
            secret: SECRET.to_string(),
        })
        .await;
        assert!(res.is_ok());
    }
}
