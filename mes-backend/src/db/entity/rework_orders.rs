use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "rework_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub rework_no: String,
    pub ncr_id: i64,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub rework_quantity: Decimal,
    pub completed_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub scrap_quantity: Decimal,
    pub unit: String,
    pub rework_type: i32,
    pub rework_reason: String,
    pub rework_plan: Option<String>,
    pub rework_process: Option<String>,
    pub workshop_id: Option<i64>,
    pub plan_start_date: Option<chrono::NaiveDate>,
    pub plan_end_date: Option<chrono::NaiveDate>,
    pub actual_start_date: Option<chrono::NaiveDate>,
    pub actual_end_date: Option<chrono::NaiveDate>,
    pub handler_id: Option<i64>,
    pub rework_cost: Decimal,
    pub rework_status: i32,
    pub inspection_result: Option<i32>,
    pub inspector_id: Option<i64>,
    pub inspection_time: Option<chrono::DateTime<chrono::Utc>>,
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
