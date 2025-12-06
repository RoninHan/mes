use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_inspections")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub inspection_no: String,
    pub equipment_id: i64,
    pub inspection_type: i16,
    pub inspection_time: DateTimeWithTimeZone,
    pub inspector_id: Option<i64>,
    pub result: i16,
    pub items: Option<Json>,
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



