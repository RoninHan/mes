use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_calibration_records")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub record_no: String,
    pub equipment_id: i64,
    pub calibration_type: i32,
    pub calibration_date: chrono::NaiveDate,
    pub calibration_institution: Option<String>,
    pub calibrator_id: Option<i64>,
    pub calibration_standard: Option<String>,
    pub calibration_result: i32,
    pub certificate_no: Option<String>,
    pub certificate_valid_date: Option<chrono::NaiveDate>,
    pub next_calibration_date: Option<chrono::NaiveDate>,
    pub calibration_cost: Decimal,
    pub deviation_before: Option<String>,
    pub deviation_after: Option<String>,
    pub adjustment_content: Option<String>,
    pub certificate_url: Option<String>,
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
