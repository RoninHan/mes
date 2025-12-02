use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "suppliers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub supplier_code: String,
    pub supplier_name: String,
    pub short_name: Option<String>,
    pub supplier_type: i8,
    pub supplier_level: Option<String>,
    pub credit_code: Option<String>,
    pub legal_person: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_mobile: Option<String>,
    pub email: Option<String>,
    pub fax: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub bank_name: Option<String>,
    pub bank_account: Option<String>,
    pub tax_rate: Option<Decimal>,
    pub payment_terms: Option<String>,
    pub delivery_cycle: Option<i32>,
    pub cooperation_start_date: Option<Date>,
    pub status: i8,
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


