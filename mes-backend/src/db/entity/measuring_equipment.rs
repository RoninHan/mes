use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "measuring_equipment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub equipment_code: String,
    pub equipment_name: String,
    pub equipment_model: Option<String>,
    pub equipment_type: i16, // 1:量具, 2:仪器, 3:检测设备, 4:其他
    pub manufacturer: Option<String>,
    pub serial_no: Option<String>,
    pub purchase_date: Option<Date>,
    pub accuracy_level: Option<String>,
    pub measurement_range: Option<String>,
    pub calibration_cycle: i32,
    pub last_calibration_date: Option<Date>,
    pub next_calibration_date: Option<Date>,
    pub calibration_institution: Option<String>,
    pub calibration_certificate_no: Option<String>,
    pub equipment_status: i16, // 1:正常, 2:待校准, 3:校准中, 4:停用, 5:报废
    pub location: Option<String>,
    pub custodian_id: Option<i64>,
    pub usage_frequency: Option<String>,
    pub maintenance_requirements: Option<String>,
    pub image_url: Option<String>,
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


