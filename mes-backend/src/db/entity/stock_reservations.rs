use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "stock_reservations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub reservation_no: String,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub reserved_quantity: Decimal,
    pub unit: String,
    pub source_type: i16,
    pub source_id: Option<i64>,
    pub status: i16,
    pub expire_time: Option<DateTimeWithTimeZone>,
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


