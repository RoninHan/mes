use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quality_inspection_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub report_id: i64,
    pub item_code: String,
    pub item_name: String,
    pub item_type: i32,
    pub inspection_method: Option<String>,
    pub standard_value: Option<String>,
    pub upper_limit: Option<Decimal>,
    pub lower_limit: Option<Decimal>,
    pub actual_value: Option<String>,
    pub unit: Option<String>,
    pub inspection_equipment: Option<String>,
    pub item_result: i32,
    pub defect_quantity: i32,
    pub defect_code: Option<String>,
    pub defect_level: Option<i32>,
    pub is_key_item: i32,
    pub sequence_no: i32,
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
