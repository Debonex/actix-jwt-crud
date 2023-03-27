use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use redis::AsyncCommands;
use redis::Client;

use crate::constants::TOKEN_VALID_TIME;
use crate::error::internal;
use crate::{constants::TOKEN_SECRET, error::ServerResult, models::token_claims::TokenClaims};

pub async fn refresh_token(client: &Client, claims: &TokenClaims) -> ServerResult<String> {
    let token = generate_token(claims).map_err(internal)?;
    let mut con = client.get_async_connection().await.map_err(internal)?;
    con.set_ex(claims.user_id, &token, TOKEN_VALID_TIME)
        .await
        .map_err(internal)?;
    Ok(token)
}

pub async fn get_token(client: &Client, user_id: i32) -> redis::RedisResult<String> {
    let mut con = client.get_async_connection().await?;
    con.get(user_id).await
}

pub fn generate_token(claims: &TokenClaims) -> jsonwebtoken::errors::Result<String> {
    jsonwebtoken::encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(TOKEN_SECRET),
    )
}

pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<TokenClaims>> {
    jsonwebtoken::decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(TOKEN_SECRET),
        &Validation::default(),
    )
}
