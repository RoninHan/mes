use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "workshops")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub workshop_code: String,
    pub workshop_name: String,
    pub workshop_type: i16,
    pub manager_id: Option<i64>,
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



