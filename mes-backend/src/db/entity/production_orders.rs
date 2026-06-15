use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "production_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub order_no: String,
    pub plan_id: Option<i64>,
    pub source_type: i32,
    pub source_order_no: Option<String>,
    pub material_id: i64,
    pub bom_id: Option<i64>,
    pub routing_id: Option<i64>,
    pub plan_quantity: Decimal,
    pub actual_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub scrap_quantity: Decimal,
    pub unit: String,
    pub priority: i32,
    pub plan_start_date: chrono::NaiveDate,
    pub plan_end_date: chrono::NaiveDate,
    pub actual_start_date: Option<chrono::NaiveDate>,
    pub actual_end_date: Option<chrono::NaiveDate>,
    pub workshop_id: Option<i64>,
    pub production_line: Option<String>,
    pub batch_no: Option<String>,
    pub customer_id: Option<i64>,
    pub order_status: i32,
    pub is_locked: i32,
    pub is_urgent: i32,
    pub standard_hours: Decimal,
    pub actual_hours: Decimal,
    pub leader_id: Option<i64>,
    pub planner_id: Option<i64>,
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
