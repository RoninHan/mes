use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_maintenance_plans")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub plan_no: String,
    pub equipment_id: i64,
    pub plan_type: i16,
    pub cycle_type: i16,
    pub cycle_value: i32,
    pub next_due_time: Option<DateTimeWithTimeZone>,
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



