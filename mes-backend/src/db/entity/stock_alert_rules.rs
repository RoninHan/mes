use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "stock_alert_rules")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub min_quantity: Option<Decimal>,
    pub max_quantity: Option<Decimal>,
    pub enabled: i16,
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


