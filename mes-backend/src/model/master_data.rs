use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub status: Option<i32>,
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
    pub status: Option<i32>,
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
    pub status: i32,
    pub remark: Option<String>,
}

// ---------- Materials ----------

#[derive(Debug, Deserialize)]
pub struct MaterialsQuery {
    pub category_id: Option<i64>,
    pub material_type: Option<i32>,
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
    pub material_type: i32,
    pub unit: String,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MaterialDto {
    pub id: i64,
    pub material_code: String,
    pub material_name: String,
    pub material_spec: Option<String>,
    pub category_id: i64,
    pub material_type: i32,
    pub unit: String,
    pub status: i32,
    pub remark: Option<String>,
}

// ---------- Suppliers ----------

#[derive(Debug, Deserialize)]
pub struct SupplierQuery {
    pub supplier_type: Option<i32>,
    pub status: Option<i32>,
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
    pub supplier_type: i32,
    pub supplier_level: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SupplierDto {
    pub id: i64,
    pub supplier_code: String,
    pub supplier_name: String,
    pub supplier_type: i32,
    pub supplier_level: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub status: i32,
    pub remark: Option<String>,
}

// ---------- Customers ----------

#[derive(Debug, Deserialize)]
pub struct CustomerQuery {
    pub customer_type: Option<i32>,
    pub status: Option<i32>,
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
    pub customer_type: i32,
    pub customer_level: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CustomerDto {
    pub id: i64,
    pub customer_code: String,
    pub customer_name: String,
    pub customer_type: i32,
    pub customer_level: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub status: i32,
    pub remark: Option<String>,
}

// ---------- Workshops ----------

#[derive(Debug, Deserialize)]
pub struct WorkshopQuery {
    pub status: Option<i32>,
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct WorkshopPayload {
    pub workshop_code: String,
    pub workshop_name: String,
    pub workshop_type: i32,
    pub manager_id: Option<i64>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WorkshopDto {
    pub id: i64,
    pub workshop_code: String,
    pub workshop_name: String,
    pub workshop_type: i32,
    pub manager_id: Option<i64>,
    pub status: i32,
    pub remark: Option<String>,
}

// ---------- Warehouses ----------

#[derive(Debug, Deserialize)]
pub struct WarehouseQuery {
    pub warehouse_type: Option<i32>,
    pub status: Option<i32>,
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct WarehousePayload {
    pub warehouse_code: String,
    pub warehouse_name: String,
    pub warehouse_type: i32,
    pub location: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WarehouseDto {
    pub id: i64,
    pub warehouse_code: String,
    pub warehouse_name: String,
    pub warehouse_type: i32,
    pub location: Option<String>,
    pub status: i32,
    pub remark: Option<String>,
}

// ---------- Locations ----------

#[derive(Debug, Deserialize)]
pub struct LocationQuery {
    pub warehouse_id: Option<i64>,
    pub status: Option<i32>,
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct LocationPayload {
    pub warehouse_id: i64,
    pub location_code: String,
    pub location_name: String,
    pub location_type: i32,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LocationDto {
    pub id: i64,
    pub warehouse_id: i64,
    pub location_code: String,
    pub location_name: String,
    pub location_type: i32,
    pub status: i32,
    pub remark: Option<String>,
}

// ---------- BOM ----------

#[derive(Debug, Deserialize)]
pub struct BomQuery {
    pub material_id: Option<i64>,
    pub status: Option<i32>,
    pub is_default: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct BomPayload {
    pub material_id: i64,
    pub bom_code: String,
    pub version: String,
    pub bom_type: i32,
    pub is_default: Option<i32>,
    pub status: Option<i32>,
    pub items: Value,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BomDto {
    pub id: i64,
    pub material_id: i64,
    pub bom_code: String,
    pub version: String,
    pub bom_type: i32,
    pub is_default: i32,
    pub status: i32,
    pub items: Value,
    pub remark: Option<String>,
}

// ---------- Process Routes ----------

#[derive(Debug, Deserialize)]
pub struct ProcessRouteQuery {
    pub material_id: Option<i64>,
    pub status: Option<i32>,
    pub is_default: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct ProcessRoutePayload {
    pub material_id: i64,
    pub route_code: String,
    pub route_name: String,
    pub version: String,
    pub is_default: Option<i32>,
    pub status: Option<i32>,
    pub operations: Value,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProcessRouteDto {
    pub id: i64,
    pub material_id: i64,
    pub route_code: String,
    pub route_name: String,
    pub version: String,
    pub is_default: i32,
    pub status: i32,
    pub operations: Value,
    pub remark: Option<String>,
}


