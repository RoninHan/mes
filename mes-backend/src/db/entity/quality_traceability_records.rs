use sea_orm::entity::prelude::*;
use chrono;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quality_traceability_records")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub trace_no: String,
    pub trace_type: i32,
    pub material_id: i64,
    pub batch_no: String,
    pub serial_no: Option<String>,
    pub production_order_no: Option<String>,
    pub sales_order_no: Option<String>,
    pub customer_id: Option<i64>,
    pub supplier_id: Option<i64>,
    pub supplier_batch_no: Option<String>,
    pub production_date: Option<chrono::NaiveDate>,
    pub inspection_report_no: Option<String>,
    pub inspection_result: Option<i32>,
    pub workshop_id: Option<i64>,
    pub production_line: Option<String>,
    pub operator_ids: Option<String>,
    pub equipment_ids: Option<String>,
    pub raw_material_info: Option<String>,
    pub process_info: Option<String>,
    pub quality_info: Option<String>,
    pub trace_reason: Option<String>,
    pub trace_result: Option<String>,
    pub trace_date: chrono::NaiveDate,
    pub tracer_id: i64,
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
