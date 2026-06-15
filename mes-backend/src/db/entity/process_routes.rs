use sea_orm::entity::prelude::*;
use chrono;
use sea_orm::prelude::Json;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "process_routes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub material_id: i64,
    pub route_code: String,
    pub route_name: String,
    pub version: String,
    pub is_default: i32,
    pub status: i32,
    pub operations: Json,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
