use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub serial_no: Option<String>,
    pub quantity: Decimal,
    pub available_quantity: Decimal,
    pub locked_quantity: Decimal,
    pub allocated_quantity: Decimal,
    pub unit: String,
    pub unit_cost: Decimal,
    pub total_cost: Decimal,
    pub production_date: Option<Date>,
    pub receipt_date: Option<Date>,
    pub expiry_date: Option<Date>,
    pub supplier_id: Option<i64>,
    pub quality_status: i8,
    pub stock_status: i8,
    pub last_in_time: Option<DateTimeWithTimeZone>,
    pub last_out_time: Option<DateTimeWithTimeZone>,
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
        panic!("No relations")
    }
}

impl ActiveModelBehavior for ActiveModel {}


