use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "batch_traceability")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub material_id: i64,
    pub batch_no: String,
    pub source_type: i32,
    pub source_ref: Option<String>,
    pub current_warehouse_id: Option<i64>,
    pub current_location_id: Option<i64>,
    pub quantity: Option<Decimal>,
    pub unit: Option<String>,
    pub status: i32,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
