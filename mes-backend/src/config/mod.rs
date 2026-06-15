use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MqttConfig {
    pub default_broker: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server_addr: String,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub mqtt: MqttConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(AppConfig {
            server_addr: std::env::var("SERVER_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| anyhow!("DATABASE_URL is required"))?,
            redis_url: std::env::var("REDIS_URL")
                .map_err(|_| anyhow!("REDIS_URL is required"))?,
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret".to_string()),
            mqtt: MqttConfig {
                default_broker: std::env::var("MQTT__DEFAULT_BROKER")
                    .unwrap_or_else(|_| "tcp://localhost:1883".to_string()),
            },
        })
    }
}
