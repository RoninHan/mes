use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_maintenance_tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub task_no: String,
    pub plan_id: Option<i64>,
    pub equipment_id: i64,
    pub task_type: i16,
    pub scheduled_time: Option<DateTimeWithTimeZone>,
    pub start_time: Option<DateTimeWithTimeZone>,
    pub end_time: Option<DateTimeWithTimeZone>,
    pub result: Option<i16>,
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



