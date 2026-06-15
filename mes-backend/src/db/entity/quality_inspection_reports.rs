use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quality_inspection_reports")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub report_no: String,
    pub task_id: i64,
    pub inspection_type: i32,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub supplier_id: Option<i64>,
    pub production_order_id: Option<i64>,
    pub inspection_date: chrono::NaiveDate,
    pub inspection_time: chrono::DateTime<chrono::Utc>,
    pub inspector_id: i64,
    pub reviewer_id: Option<i64>,
    pub review_time: Option<chrono::DateTime<chrono::Utc>>,
    pub inspection_quantity: Decimal,
    pub sample_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub unit: String,
    pub qualified_rate: Decimal,
    pub inspection_result: i32,
    pub disposition: Option<i32>,
    pub major_defects: i32,
    pub minor_defects: i32,
    pub critical_defects: i32,
    pub inspection_environment: Option<String>,
    pub inspection_equipment: Option<String>,
    pub report_status: i32,
    pub conclusion: Option<String>,
    pub improvement_suggestions: Option<String>,
    pub attachment_url: Option<String>,
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
