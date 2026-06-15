use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "customer_complaints")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub complaint_no: String,
    pub customer_id: i64,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub sales_order_no: Option<String>,
    pub production_order_no: Option<String>,
    pub complaint_type: i32,
    pub complaint_level: i32,
    pub complaint_date: chrono::NaiveDate,
    pub complaint_time: chrono::DateTime<chrono::Utc>,
    pub complaint_quantity: Option<Decimal>,
    pub unit: Option<String>,
    pub complaint_description: String,
    pub defect_description: Option<String>,
    pub defect_images: Option<String>,
    pub customer_requirement: Option<String>,
    pub receiver_id: i64,
    pub handler_id: Option<i64>,
    pub response_deadline: Option<chrono::NaiveDate>,
    pub response_time: Option<chrono::DateTime<chrono::Utc>>,
    pub response_content: Option<String>,
    pub root_cause_analysis: Option<String>,
    pub corrective_action: Option<String>,
    pub preventive_action: Option<String>,
    pub compensation_amount: Decimal,
    pub processing_cost: Decimal,
    pub complaint_status: i32,
    pub is_valid: i32,
    pub customer_satisfaction: Option<i32>,
    pub closure_date: Option<chrono::NaiveDate>,
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
