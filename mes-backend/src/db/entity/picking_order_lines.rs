use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "picking_order_lines")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub picking_id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: Decimal,
    pub actual_quantity: Decimal,
    pub unit: String,
    pub line_status: i32,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
