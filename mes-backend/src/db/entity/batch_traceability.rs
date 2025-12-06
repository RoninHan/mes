use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "batch_traceability")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub material_id: i64,
    pub batch_no: String,
    pub source_type: i16,
    pub source_ref: Option<String>,
    pub current_warehouse_id: Option<i64>,
    pub current_location_id: Option<i64>,
    pub quantity: Option<Decimal>,
    pub unit: Option<String>,
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


