use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "materials")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub material_code: String,
    pub material_name: String,
    pub material_spec: Option<String>,
    pub material_model: Option<String>,
    pub category_id: i64,
    pub material_type: i8,
    pub unit: String,
    pub aux_unit: Option<String>,
    pub conversion_rate: Option<Decimal>,
    pub barcode: Option<String>,
    pub qr_code: Option<String>,
    pub abc_category: Option<String>,
    pub batch_managed: i8,
    pub serial_managed: i8,
    pub shelf_life_days: Option<i32>,
    pub min_stock: Option<Decimal>,
    pub max_stock: Option<Decimal>,
    pub safety_stock: Option<Decimal>,
    pub standard_cost: Option<Decimal>,
    pub purchase_price: Option<Decimal>,
    pub sales_price: Option<Decimal>,
    pub lead_time: Option<i32>,
    pub drawing_no: Option<String>,
    pub version: Option<String>,
    pub weight: Option<Decimal>,
    pub volume: Option<Decimal>,
    pub color: Option<String>,
    pub quality_level: Option<String>,
    pub origin_place: Option<String>,
    pub manufacturer: Option<String>,
    pub supplier_id: Option<i64>,
    pub status: i8,
    pub image_url: Option<String>,
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
        panic!("No Relation")
    }
}

impl ActiveModelBehavior for ActiveModel {}


