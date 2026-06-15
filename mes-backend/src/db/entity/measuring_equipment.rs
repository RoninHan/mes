use sea_orm::entity::prelude::*;
use chrono;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "measuring_equipment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub equipment_code: String,
    pub equipment_name: String,
    pub equipment_model: Option<String>,
    pub equipment_type: i32,
    pub manufacturer: Option<String>,
    pub serial_no: Option<String>,
    pub purchase_date: Option<chrono::NaiveDate>,
    pub accuracy_level: Option<String>,
    pub measurement_range: Option<String>,
    pub calibration_cycle: i32,
    pub last_calibration_date: Option<chrono::NaiveDate>,
    pub next_calibration_date: Option<chrono::NaiveDate>,
    pub calibration_institution: Option<String>,
    pub calibration_certificate_no: Option<String>,
    pub equipment_status: i32,
    pub location: Option<String>,
    pub custodian_id: Option<i64>,
    pub usage_frequency: Option<String>,
    pub maintenance_requirements: Option<String>,
    pub image_url: Option<String>,
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
