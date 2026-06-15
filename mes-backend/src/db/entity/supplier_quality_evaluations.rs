use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "supplier_quality_evaluations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub evaluation_no: String,
    pub supplier_id: i64,
    pub evaluation_period: String,
    pub evaluation_date: chrono::NaiveDate,
    pub evaluator_id: i64,
    pub total_receipts: i32,
    pub total_quantity: Decimal,
    pub qualified_receipts: i32,
    pub qualified_quantity: Decimal,
    pub unqualified_receipts: i32,
    pub unqualified_quantity: Decimal,
    pub batch_qualified_rate: Decimal,
    pub quantity_qualified_rate: Decimal,
    pub on_time_delivery_rate: Decimal,
    pub response_speed_score: Decimal,
    pub service_attitude_score: Decimal,
    pub quality_score: Decimal,
    pub delivery_score: Decimal,
    pub service_score: Decimal,
    pub total_score: Decimal,
    pub evaluation_level: String,
    pub major_issues: Option<String>,
    pub improvement_requirements: Option<String>,
    pub evaluation_conclusion: Option<String>,
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
