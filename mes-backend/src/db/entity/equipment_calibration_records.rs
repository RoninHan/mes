use sea_orm::entity::prelude::*;
use sea_orm::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_calibration_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub record_no: String,
    pub equipment_id: i64,
    pub calibration_type: i16, // 1:内部校准, 2:外部校准
    pub calibration_date: Date,
    pub calibration_institution: Option<String>,
    pub calibrator_id: Option<i64>,
    pub calibration_standard: Option<String>,
    pub calibration_result: i16, // 1:合格, 2:不合格
    pub certificate_no: Option<String>,
    pub certificate_valid_date: Option<Date>,
    pub next_calibration_date: Option<Date>,
    pub calibration_cost: Decimal,
    pub deviation_before: Option<String>,
    pub deviation_after: Option<String>,
    pub adjustment_content: Option<String>,
    pub certificate_url: Option<String>,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_by: Option<i64>,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations")
    }
}

impl ActiveModelBehavior for ActiveModel {}


