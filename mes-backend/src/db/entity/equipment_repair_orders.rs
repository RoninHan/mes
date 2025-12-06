use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_repair_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub repair_no: String,
    pub fault_id: Option<i64>,
    pub equipment_id: i64,
    pub repair_type: i16,
    pub start_time: Option<DateTimeWithTimeZone>,
    pub end_time: Option<DateTimeWithTimeZone>,
    pub downtime_minutes: Option<i32>,
    pub repair_person_id: Option<i64>,
    pub cost_labor: Decimal,
    pub cost_spare_parts: Decimal,
    pub status: i16,
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



