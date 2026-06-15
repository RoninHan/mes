use sea_orm::entity::prelude::*;
use chrono;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "login_logs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub login_time: chrono::DateTime<chrono::Utc>,
    pub login_ip: Option<String>,
    pub user_agent: Option<String>,
    pub result: i32,
    pub fail_reason: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
