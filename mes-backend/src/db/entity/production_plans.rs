use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "production_plans")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub plan_no: String,
    pub plan_name: String,
    pub plan_type: i32,
    pub plan_period: Option<String>,
    pub plan_start_date: chrono::NaiveDate,
    pub plan_end_date: chrono::NaiveDate,
    pub total_orders: i32,
    pub completed_orders: i32,
    pub total_quantity: Decimal,
    pub completed_quantity: Decimal,
    pub plan_status: i32,
    pub completion_rate: Decimal,
    pub planner_id: Option<i64>,
    pub dept_id: Option<i64>,
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
