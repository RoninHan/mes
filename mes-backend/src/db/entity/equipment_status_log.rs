use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_status_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub equipment_id: i32,
    pub status: i16,
    pub running_param: Option<Json>,
    pub error_code: Option<String>,
    pub error_desc: Option<String>,
    pub log_time: DateTimeWithTimeZone,
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


