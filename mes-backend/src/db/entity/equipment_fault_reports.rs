use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_fault_reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub fault_no: String,
    pub equipment_id: i64,
    pub fault_level: i16,
    pub occur_time: DateTimeWithTimeZone,
    pub report_time: DateTimeWithTimeZone,
    pub reporter_id: Option<i64>,
    pub description: Option<String>,
    pub status: i16,
    pub root_cause: Option<String>,
    pub solution: Option<String>,
    pub remark: Option<String>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No Relation")
    }
}

impl ActiveModelBehavior for ActiveModel {}



