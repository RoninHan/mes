use sea_orm::entity::prelude::*;
use sea_orm::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quality_inspection_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub report_id: i64,
    pub item_code: String,
    pub item_name: String,
    pub item_type: i16, // 1:尺寸, 2:外观, 3:性能, 4:功能, 5:其他
    pub inspection_method: Option<String>,
    pub standard_value: Option<String>,
    pub upper_limit: Option<Decimal>,
    pub lower_limit: Option<Decimal>,
    pub actual_value: Option<String>,
    pub unit: Option<String>,
    pub inspection_equipment: Option<String>,
    pub item_result: i16, // 1:合格, 2:不合格
    pub defect_quantity: i32,
    pub defect_code: Option<String>,
    pub defect_level: Option<i16>, // 1:致命, 2:严重, 3:一般, 4:轻微
    pub is_key_item: i16,
    pub sequence_no: i32,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_by: Option<i64>,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations")
    }
}

impl ActiveModelBehavior for ActiveModel {}


