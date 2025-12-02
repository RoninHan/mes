use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inbound_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub inbound_no: String,
    pub inbound_type: i8,
    pub source_order_no: Option<String>,
    pub warehouse_id: i64,
    pub supplier_id: Option<i64>,
    pub delivery_no: Option<String>,
    pub plan_inbound_date: Option<Date>,
    pub actual_inbound_date: Option<Date>,
    pub total_quantity: Decimal,
    pub total_amount: Decimal,
    pub handler_id: Option<i64>,
    pub receiver_id: Option<i64>,
    pub inspector_id: Option<i64>,
    pub inspect_result: Option<i8>,
    pub order_status: i8,
    pub is_urgent: i8,
    pub dept_id: Option<i64>,
    pub remark: Option<String>,
    pub attachment_url: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_by: Option<i64>,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i8,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Details,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Details => Entity::has_many(super::inbound_order_details::Entity).into(),
        }
    }
}

impl Related<super::inbound_order_details::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Details.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


