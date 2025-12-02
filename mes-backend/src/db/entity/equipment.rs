use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub equipment_code: String,
    pub equipment_name: String,
    pub equipment_type: String,
    pub model: Option<String>,
    pub factory: Option<String>,
    pub production_date: Option<Date>,
    pub install_date: Option<Date>,
    pub status: i16,
    pub ip_address: Option<String>,
    pub mqtt_topic: String,
    pub location: Option<String>,
    pub responsible_person: Option<String>,
    pub remark: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    EquipmentStatusLog,
    EquipmentMqttConfig,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::EquipmentStatusLog => Entity::has_many(super::equipment_status_log::Entity).into(),
            Self::EquipmentMqttConfig => {
                Entity::has_one(super::equipment_mqtt_config::Entity).into()
            }
        }
    }
}

impl Related<super::equipment_status_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EquipmentStatusLog.def()
    }
}

impl Related<super::equipment_mqtt_config::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EquipmentMqttConfig.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


