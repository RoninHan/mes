use sea_orm::entity::prelude::*;
use chrono;
use sea_orm::prelude::Json;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_status_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub equipment_id: i32,
    pub status: i32,
    pub running_param: Option<Json>,
    pub error_code: Option<String>,
    pub error_desc: Option<String>,
    pub log_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
