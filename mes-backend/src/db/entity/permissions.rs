use sea_orm::entity::prelude::*;
use chrono;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub permission_code: String,
    pub permission_name: String,
    pub parent_id: i64,
    pub permission_type: i32,
    pub route_path: Option<String>,
    pub component_path: Option<String>,
    pub icon: Option<String>,
    pub api_url: Option<String>,
    pub api_method: Option<String>,
    pub sort_order: i32,
    pub is_visible: i32,
    pub status: i32,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_by: Option<i64>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
