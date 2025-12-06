use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "stock_adjustments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub adjustment_no: String,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub quantity_delta: Decimal,
    pub unit: String,
    pub reason: Option<String>,
    pub adjustment_type: i16,
    pub business_time: DateTimeWithTimeZone,
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


