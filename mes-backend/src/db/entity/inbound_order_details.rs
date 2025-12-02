use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inbound_order_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub inbound_order_id: i64,
    pub material_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub serial_no: Option<String>,
    pub plan_quantity: Decimal,
    pub actual_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub unit: String,
    pub unit_price: Decimal,
    pub amount: Decimal,
    pub production_date: Option<Date>,
    pub expiry_date: Option<Date>,
    pub quality_status: i8,
    pub line_status: i8,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_by: Option<i64>,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i8,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    InboundOrder,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::InboundOrder => Entity::belongs_to(super::inbound_orders::Entity)
                .from(Column::InboundOrderId)
                .to(super::inbound_orders::Column::Id)
                .into(),
        }
    }
}

impl Related<super::inbound_orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InboundOrder.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


