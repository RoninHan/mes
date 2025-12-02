use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "departments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub dept_code: String,
    pub dept_name: String,
    pub parent_id: i64,
    pub dept_level: i32,
    pub dept_path: Option<String>,
    pub manager_id: Option<i64>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub sort_order: i32,
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


