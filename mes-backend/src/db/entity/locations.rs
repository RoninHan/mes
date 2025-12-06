use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "locations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub warehouse_id: i64,
    pub location_code: String,
    pub location_name: String,
    pub location_type: i16,
    pub status: i16,
    pub remark: Option<String>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::warehouses::Entity",
        from = "Column::WarehouseId",
        to = "super::warehouses::Column::Id"
    )]
    Warehouses,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Warehouses => Entity::belongs_to(super::warehouses::Entity)
                .from(Column::WarehouseId)
                .to(super::warehouses::Column::Id)
                .into(),
        }
    }
}

impl Related<super::warehouses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Warehouses.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}



