use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "nonconforming_products")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub ncr_no: String,
    pub report_id: Option<i64>,
    pub source_type: i32,
    pub source_order_no: Option<String>,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub serial_no: Option<String>,
    pub supplier_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub production_order_id: Option<i64>,
    pub work_order_id: Option<i64>,
    pub process_id: Option<i64>,
    pub defect_quantity: Decimal,
    pub unit: String,
    pub defect_code: Option<String>,
    pub defect_name: Option<String>,
    pub defect_level: i32,
    pub defect_description: Option<String>,
    pub defect_location: Option<String>,
    pub defect_images: Option<String>,
    pub found_date: chrono::NaiveDate,
    pub found_time: chrono::DateTime<chrono::Utc>,
    pub finder_id: i64,
    pub responsible_dept_id: Option<i64>,
    pub responsible_person_id: Option<i64>,
    pub root_cause: Option<String>,
    pub disposition: Option<i32>,
    pub disposition_quantity: Decimal,
    pub disposition_date: Option<chrono::NaiveDate>,
    pub disposition_handler_id: Option<i64>,
    pub disposition_result: Option<String>,
    pub rework_order_no: Option<String>,
    pub corrective_action: Option<String>,
    pub preventive_action: Option<String>,
    pub ncr_status: i32,
    pub is_repetitive: i32,
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
