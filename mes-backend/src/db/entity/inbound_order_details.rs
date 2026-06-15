use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inbound_order_details")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub inbound_order_id: i64,
    pub material_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub serial_no: Option<String>,
    pub plan_quantity: Decimal,
    pub actual_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub unit: String,
    pub unit_price: Decimal,
    pub amount: Decimal,
    pub production_date: Option<chrono::NaiveDate>,
    pub expiry_date: Option<chrono::NaiveDate>,
    pub quality_status: i32,
    pub line_status: i32,
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
