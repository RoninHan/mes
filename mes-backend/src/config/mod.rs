use anyhow::Result;
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
        let mut cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;

        // Provide some defaults for local development
        cfg.set_default("server_addr", "0.0.0.0:8080")?;
        cfg.set_default("mqtt.default_broker", "tcp://localhost:1883")?;

        let s: AppConfig = cfg.try_deserialize()?;
        Ok(s)
    }
}


