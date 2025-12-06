use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "boms")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub material_id: i64,
    pub bom_code: String,
    pub version: String,
    pub bom_type: i16,
    pub is_default: i16,
    pub status: i16,
    pub items: Json,
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



