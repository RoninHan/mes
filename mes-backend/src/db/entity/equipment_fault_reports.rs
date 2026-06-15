use sea_orm::entity::prelude::*;
use chrono;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_fault_reports")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub fault_no: String,
    pub equipment_id: i64,
    pub fault_level: i32,
    pub occur_time: chrono::DateTime<chrono::Utc>,
    pub report_time: chrono::DateTime<chrono::Utc>,
    pub reporter_id: Option<i64>,
    pub description: Option<String>,
    pub status: i32,
    pub root_cause: Option<String>,
    pub solution: Option<String>,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
