use anyhow::Result;
use sea_orm::{Database as SeaDatabase, DatabaseConnection};
use std::sync::Arc;

pub mod entity;
pub mod dao;

#[derive(Clone)]
pub struct Database {
    pub conn: Arc<DatabaseConnection>,
}

impl Database {
    pub async fn connect(url: &str) -> Result<Self> {
        let conn = SeaDatabase::connect(url).await?;
        Ok(Self {
            conn: Arc::new(conn),
        })
    }

    pub fn conn(&self) -> &DatabaseConnection {
        &self.conn
    }
}



