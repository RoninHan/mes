use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "process_routes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub material_id: i64,
    pub route_code: String,
    pub route_name: String,
    pub version: String,
    pub is_default: i16,
    pub status: i16,
    pub operations: Json,
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



