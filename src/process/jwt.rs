use crate::{SignClaimOpts, VerifyOpts};
use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use tracing::info;

pub async fn process_encode_jwt(claims: &SignClaimOpts) -> Result<()> {
    info!("encode secert {:?}", claims.secret);
    let secret: &[u8] = claims.secret.as_bytes();
    let token = encode::<SignClaimOpts>(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(secret),
    )?;
    println!("{:?}", token);
    Ok(())
}

pub async fn process_decode_jwt(opts: &VerifyOpts) -> Result<()> {
    info!("decode secert {:?}", opts.secret);
    let token = decode::<SignClaimOpts>(
        &opts.token,
        &DecodingKey::from_secret(opts.secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    println!("{:?}", token.claims);
    Ok(())
}
