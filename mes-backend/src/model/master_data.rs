use serde::{Deserialize, Serialize};

fn default_page() -> u64 {
    0
}

fn default_page_size() -> u64 {
    20
}

#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total: u64,
}

// ---------- Material Categories ----------

#[derive(Debug, Deserialize)]
pub struct CategoryQuery {
    pub parent_id: Option<i64>,
    pub status: Option<i8>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct CategoryPayload {
    pub category_code: String,
    pub category_name: String,
    pub parent_id: Option<i64>,
    pub category_level: Option<i32>,
    pub sort_order: Option<i32>,
    pub status: Option<i8>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CategoryDto {
    pub id: i64,
    pub category_code: String,
    pub category_name: String,
    pub parent_id: i64,
    pub category_level: i32,
    pub sort_order: i32,
    pub status: i8,
    pub remark: Option<String>,
}

// ---------- Materials ----------

#[derive(Debug, Deserialize)]
pub struct MaterialsQuery {
    pub category_id: Option<i64>,
    pub material_type: Option<i8>,
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct MaterialPayload {
    pub material_code: String,
    pub material_name: String,
    pub material_spec: Option<String>,
    pub material_model: Option<String>,
    pub category_id: i64,
    pub material_type: i8,
    pub unit: String,
    pub status: Option<i8>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MaterialDto {
    pub id: i64,
    pub material_code: String,
    pub material_name: String,
    pub material_spec: Option<String>,
    pub category_id: i64,
    pub material_type: i8,
    pub unit: String,
    pub status: i8,
    pub remark: Option<String>,
}

// ---------- Suppliers ----------

#[derive(Debug, Deserialize)]
pub struct SupplierQuery {
    pub supplier_type: Option<i8>,
    pub status: Option<i8>,
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct SupplierPayload {
    pub supplier_code: String,
    pub supplier_name: String,
    pub supplier_type: i8,
    pub supplier_level: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub status: Option<i8>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SupplierDto {
    pub id: i64,
    pub supplier_code: String,
    pub supplier_name: String,
    pub supplier_type: i8,
    pub supplier_level: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub status: i8,
    pub remark: Option<String>,
}

// ---------- Customers ----------

#[derive(Debug, Deserialize)]
pub struct CustomerQuery {
    pub customer_type: Option<i8>,
    pub status: Option<i8>,
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct CustomerPayload {
    pub customer_code: String,
    pub customer_name: String,
    pub customer_type: i8,
    pub customer_level: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub status: Option<i8>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CustomerDto {
    pub id: i64,
    pub customer_code: String,
    pub customer_name: String,
    pub customer_type: i8,
    pub customer_level: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub status: i8,
    pub remark: Option<String>,
}


