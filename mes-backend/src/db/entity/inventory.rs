use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub serial_no: Option<String>,
    pub quantity: Decimal,
    pub available_quantity: Decimal,
    pub locked_quantity: Decimal,
    pub allocated_quantity: Decimal,
    pub unit: String,
    pub unit_cost: Decimal,
    pub total_cost: Decimal,
    pub production_date: Option<chrono::NaiveDate>,
    pub receipt_date: Option<chrono::NaiveDate>,
    pub expiry_date: Option<chrono::NaiveDate>,
    pub supplier_id: Option<i64>,
    pub quality_status: i32,
    pub stock_status: i32,
    pub last_in_time: Option<chrono::DateTime<chrono::Utc>>,
    pub last_out_time: Option<chrono::DateTime<chrono::Utc>>,
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
