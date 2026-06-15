use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "return_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub return_no: String,
    pub production_order_id: i64,
    pub warehouse_id: i64,
    pub work_order_id: Option<i64>,
    pub return_type: i32,
    pub plan_return_date: Option<chrono::NaiveDate>,
    pub actual_return_date: Option<chrono::NaiveDate>,
    pub total_quantity: Decimal,
    pub order_status: i32,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
