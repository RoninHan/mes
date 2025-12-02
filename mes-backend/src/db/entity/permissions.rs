use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "permissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub permission_code: String,
    pub permission_name: String,
    pub parent_id: i64,
    pub permission_type: i8,
    pub route_path: Option<String>,
    pub component_path: Option<String>,
    pub icon: Option<String>,
    pub api_url: Option<String>,
    pub api_method: Option<String>,
    pub sort_order: i32,
    pub is_visible: i8,
    pub status: i8,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_by: Option<i64>,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i8,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relation")
    }
}

impl ActiveModelBehavior for ActiveModel {}


