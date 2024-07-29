use crate::{SignClaimOpts, VerifyOpts};
use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: u64,
}

pub async fn process_encode_jwt(opts: &SignClaimOpts) -> Result<String> {
    info!("encode secert {:?}", opts.secret);
    let secret: &[u8] = opts.secret.as_bytes();
    let header = Header {
        kid: Some(opts.secret.clone()),
        alg: Algorithm::HS512,
        ..Default::default()
    };
    let my_claims = Claims {
        sub: opts.sub.clone(),
        company: opts.aud.clone(),
        exp: opts.exp as u64,
    };
    let token = encode::<Claims>(&header, &my_claims, &EncodingKey::from_secret(secret))?;
    println!("{:?}", token);
    Ok(token)
}

pub async fn process_decode_jwt(opts: &VerifyOpts) -> Result<Claims> {
    info!("decode secert {:?}", opts.secret);
    let token = decode::<Claims>(
        &opts.token,
        &DecodingKey::from_secret(opts.secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    )?;
    println!("{:?}", token.claims);
    Ok(token.claims)
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_jwt() -> Result<()> {
        const SECRET: &str = "asdiuop123";
        let claims = SignClaimOpts {
            exp: 10000000000,
            secret: SECRET.to_string(),
            sub: "sub".to_string(),
            aud: "aud".to_string(),
        };

        let token = process_encode_jwt(&claims).await?;
        let res = process_decode_jwt(&VerifyOpts {
            token: token.clone(),
            secret: SECRET.to_string(),
        })
        .await?;
        assert_eq!(res.sub, claims.sub);
        assert_eq!(res.company, "aud");
        Ok(())
    }
}
