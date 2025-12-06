use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "operation_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub module: Option<String>,
    pub action: Option<String>,
    pub request_path: Option<String>,
    pub method: Option<String>,
    pub request_time: DateTimeWithTimeZone,
    pub success: i16,
    pub client_ip: Option<String>,
    pub payload: Option<Json>,
    pub error_message: Option<String>,
    pub created_time: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No Relation")
    }
}

impl ActiveModelBehavior for ActiveModel {}



