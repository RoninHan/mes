use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "production_reports")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub report_no: String,
    pub work_order_id: i64,
    pub production_order_id: i64,
    pub process_id: i64,
    pub material_id: i64,
    pub equipment_id: Option<i64>,
    pub workshop_id: Option<i64>,
    pub report_type: i32,
    pub report_date: chrono::NaiveDate,
    pub report_time: chrono::DateTime<chrono::Utc>,
    pub shift: Option<String>,
    pub operator_id: i64,
    pub team_members: Option<String>,
    pub report_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub scrap_quantity: Decimal,
    pub unit: String,
    pub work_hours: Decimal,
    pub standard_hours: Decimal,
    pub efficiency: Decimal,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub downtime_minutes: i32,
    pub downtime_reason: Option<String>,
    pub quality_issue: Option<String>,
    pub is_approved: i32,
    pub approver_id: Option<i64>,
    pub approval_time: Option<chrono::DateTime<chrono::Utc>>,
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
