use sea_orm::entity::prelude::*;
use chrono;
use sea_orm::prelude::Json;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_inspections")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub inspection_no: String,
    pub equipment_id: i64,
    pub inspection_type: i32,
    pub inspection_time: chrono::DateTime<chrono::Utc>,
    pub inspector_id: Option<i64>,
    pub result: i32,
    pub items: Option<Json>,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
