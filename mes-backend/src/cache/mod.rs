use anyhow::Result;
use redis::{aio::ConnectionManager, Client};
use std::sync::Arc;

#[derive(Clone)]
pub struct RedisCache {
    client: Client,
    manager: Arc<ConnectionManager>,
}

impl RedisCache {
    pub async fn connect(url: &str) -> Result<Self> {
        let client = Client::open(url)?;
        let manager = ConnectionManager::new(client.clone()).await?;
        Ok(Self {
            client,
            manager: Arc::new(manager),
        })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn manager(&self) -> Arc<ConnectionManager> {
        self.manager.clone()
    }
}

pub const EQUIPMENT_STATUS_KEY_PREFIX: &str = "equipment:status:";


