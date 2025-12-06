use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "material_requirements")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub production_order_id: i64,
    pub material_id: i64,
    pub required_quantity: Decimal,
    pub reserved_quantity: Decimal,
    pub issued_quantity: Decimal,
    pub unit: String,
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


