use crate::db::entity::users;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub roles: Vec<String>,
    pub exp: usize,
}

pub fn encode_token(claims: &Claims, secret: &str) -> jsonwebtoken::errors::Result<String> {
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), claims, &key)
}

pub fn decode_token(token: &str, secret: &str) -> jsonwebtoken::errors::Result<Claims> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let data = decode::<Claims>(token, &key, &Validation::default())?;
    Ok(data.claims)
}


