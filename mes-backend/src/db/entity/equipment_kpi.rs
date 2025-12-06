use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_kpi")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub equipment_id: i64,
    pub stat_date: Date,
    pub runtime_minutes: Option<i32>,
    pub downtime_minutes: Option<i32>,
    pub fault_count: Option<i32>,
    pub mtbf_minutes: Option<i32>,
    pub mttr_minutes: Option<i32>,
    pub oee: Option<Decimal>,
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



