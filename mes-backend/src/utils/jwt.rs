use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// SSO 颁发的 JWT Claims（与 SSO / ERP 后端保持一致）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// 用户 ID
    pub sub: i64,
    pub username: String,
    /// 角色编码列表
    #[serde(default)]
    pub roles: Vec<String>,
    /// 权限码列表
    #[serde(default)]
    pub permissions: Vec<String>,
    /// Session ID（用于即时吊销）
    #[serde(default)]
    pub sid: String,
    pub exp: i64,
    #[serde(default)]
    pub iat: i64,
}

/// 生成 JWT Token
pub fn encode_token(claims: &Claims, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), claims, &key)
}

/// 验证 SSO 颁发的 JWT（使用与 SSO 相同的 JWT_SECRET 环境变量）
pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let key = DecodingKey::from_secret(secret.as_bytes());
    let data = decode::<Claims>(token, &key, &Validation::default())?;
    Ok(data.claims)
}


