use sea_orm::entity::prelude::*;
use chrono;
use sea_orm::prelude::Json;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "operation_logs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub module: Option<String>,
    pub action: Option<String>,
    pub request_path: Option<String>,
    pub method: Option<String>,
    pub request_time: chrono::DateTime<chrono::Utc>,
    pub success: i32,
    pub client_ip: Option<String>,
    pub payload: Option<Json>,
    pub error_message: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
