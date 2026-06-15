use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inbound_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub inbound_no: String,
    pub inbound_type: i32,
    pub source_order_no: Option<String>,
    pub warehouse_id: i64,
    pub supplier_id: Option<i64>,
    pub delivery_no: Option<String>,
    pub plan_inbound_date: Option<chrono::NaiveDate>,
    pub actual_inbound_date: Option<chrono::NaiveDate>,
    pub total_quantity: Decimal,
    pub total_amount: Decimal,
    pub handler_id: Option<i64>,
    pub receiver_id: Option<i64>,
    pub inspector_id: Option<i64>,
    pub inspect_result: Option<i32>,
    pub order_status: i32,
    pub is_urgent: i32,
    pub dept_id: Option<i64>,
    pub remark: Option<String>,
    pub attachment_url: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_by: Option<i64>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
