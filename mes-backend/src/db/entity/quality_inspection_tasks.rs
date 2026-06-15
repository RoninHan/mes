use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quality_inspection_tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub task_no: String,
    pub inspection_type: i32,
    pub source_type: i32,
    pub source_order_no: String,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub supplier_id: Option<i64>,
    pub production_order_id: Option<i64>,
    pub work_order_id: Option<i64>,
    pub process_id: Option<i64>,
    pub standard_id: Option<i64>,
    pub inspection_quantity: Decimal,
    pub sample_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub unit: String,
    pub inspection_level: Option<String>,
    pub aql: Option<Decimal>,
    pub sampling_plan: Option<String>,
    pub plan_start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub plan_end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub actual_start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub actual_end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub inspector_id: Option<i64>,
    pub task_status: i32,
    pub inspection_result: Option<i32>,
    pub is_urgent: i32,
    pub priority: i32,
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
