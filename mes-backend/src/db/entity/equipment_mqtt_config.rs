use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_mqtt_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub equipment_id: i32,
    pub broker_address: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: String,
    pub keep_alive: Option<i32>,
    pub qos: Option<i16>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Equipment,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Equipment => Entity::belongs_to(super::equipment::Entity)
                .from(Column::EquipmentId)
                .to(super::equipment::Column::Id)
                .into(),
        }
    }
}

impl Related<super::equipment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Equipment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


