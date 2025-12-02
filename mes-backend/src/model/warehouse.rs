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

// Inventory
#[derive(Debug, Deserialize)]
pub struct InventoryQuery {
    pub material_id: Option<i64>,
    pub warehouse_id: Option<i64>,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
pub struct InventoryDto {
    pub id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub serial_no: Option<String>,
    pub quantity: f64,
    pub available_quantity: f64,
    pub locked_quantity: f64,
    pub unit: String,
}

// Inbound Orders
#[derive(Debug, Deserialize)]
pub struct InboundQuery {
    pub warehouse_id: Option<i64>,
    pub inbound_type: Option<i8>,
    pub order_status: Option<i8>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
pub struct InboundSummaryDto {
    pub id: i64,
    pub inbound_no: String,
    pub inbound_type: i8,
    pub warehouse_id: i64,
    pub supplier_id: Option<i64>,
    pub plan_inbound_date: Option<chrono::NaiveDate>,
    pub actual_inbound_date: Option<chrono::NaiveDate>,
    pub total_quantity: f64,
    pub order_status: i8,
}

#[derive(Debug, Deserialize)]
pub struct InboundDetailPayload {
    pub material_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: f64,
    pub unit: String,
    pub unit_price: f64,
}

#[derive(Debug, Deserialize)]
pub struct InboundPayload {
    pub inbound_no: String,
    pub inbound_type: i8,
    pub warehouse_id: i64,
    pub supplier_id: Option<i64>,
    pub plan_inbound_date: Option<chrono::NaiveDate>,
    pub remark: Option<String>,
    pub details: Vec<InboundDetailPayload>,
}

#[derive(Debug, Serialize)]
pub struct InboundDetailDto {
    pub id: i64,
    pub material_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: f64,
    pub actual_quantity: f64,
    pub unit: String,
    pub unit_price: f64,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct InboundWithDetailsDto {
    pub header: InboundSummaryDto,
    pub details: Vec<InboundDetailDto>,
}


