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
    pub inbound_type: Option<i32>,
    pub order_status: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
pub struct InboundSummaryDto {
    pub id: i64,
    pub inbound_no: String,
    pub inbound_type: i32,
    pub warehouse_id: i64,
    pub supplier_id: Option<i64>,
    pub plan_inbound_date: Option<chrono::NaiveDate>,
    pub actual_inbound_date: Option<chrono::NaiveDate>,
    pub total_quantity: f64,
    pub order_status: i32,
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
    pub inbound_type: i32,
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

// ---------- Outbound Orders ----------

#[derive(Debug, Deserialize)]
pub struct OutboundQuery {
    pub warehouse_id: Option<i64>,
    pub outbound_type: Option<i32>,
    pub order_status: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
pub struct OutboundSummaryDto {
    pub id: i64,
    pub outbound_no: String,
    pub outbound_type: i32,
    pub warehouse_id: i64,
    pub customer_id: Option<i64>,
    pub plan_outbound_date: Option<chrono::NaiveDate>,
    pub actual_outbound_date: Option<chrono::NaiveDate>,
    pub total_quantity: f64,
    pub order_status: i32,
}

#[derive(Debug, Deserialize)]
pub struct OutboundDetailPayload {
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: f64,
    pub unit: String,
}

#[derive(Debug, Deserialize)]
pub struct OutboundPayload {
    pub outbound_no: String,
    pub outbound_type: i32,
    pub warehouse_id: i64,
    pub customer_id: Option<i64>,
    pub plan_outbound_date: Option<chrono::NaiveDate>,
    pub remark: Option<String>,
    pub details: Vec<OutboundDetailPayload>,
}

#[derive(Debug, Serialize)]
pub struct OutboundDetailDto {
    pub id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: f64,
    pub actual_quantity: f64,
    pub unit: String,
}

#[derive(Debug, Serialize)]
pub struct OutboundWithDetailsDto {
    pub header: OutboundSummaryDto,
    pub details: Vec<OutboundDetailDto>,
}

// ---------- Transfer Orders ----------

#[derive(Debug, Deserialize)]
pub struct TransferQuery {
    pub from_warehouse_id: Option<i64>,
    pub to_warehouse_id: Option<i64>,
    pub order_status: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
pub struct TransferSummaryDto {
    pub id: i64,
    pub transfer_no: String,
    pub from_warehouse_id: i64,
    pub to_warehouse_id: i64,
    pub plan_transfer_date: Option<chrono::NaiveDate>,
    pub actual_transfer_date: Option<chrono::NaiveDate>,
    pub total_quantity: f64,
    pub order_status: i32,
}

#[derive(Debug, Deserialize)]
pub struct TransferDetailPayload {
    pub material_id: i64,
    pub from_warehouse_id: i64,
    pub from_location_id: Option<i64>,
    pub to_warehouse_id: i64,
    pub to_location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: f64,
    pub unit: String,
}

#[derive(Debug, Deserialize)]
pub struct TransferPayload {
    pub transfer_no: String,
    pub from_warehouse_id: i64,
    pub to_warehouse_id: i64,
    pub plan_transfer_date: Option<chrono::NaiveDate>,
    pub remark: Option<String>,
    pub details: Vec<TransferDetailPayload>,
}

#[derive(Debug, Serialize)]
pub struct TransferDetailDto {
    pub id: i64,
    pub material_id: i64,
    pub from_warehouse_id: i64,
    pub from_location_id: Option<i64>,
    pub to_warehouse_id: i64,
    pub to_location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: f64,
    pub actual_quantity: f64,
    pub unit: String,
}

#[derive(Debug, Serialize)]
pub struct TransferWithDetailsDto {
    pub header: TransferSummaryDto,
    pub details: Vec<TransferDetailDto>,
}

// ---------- Stock Count ----------

#[derive(Debug, Deserialize)]
pub struct StockCountQuery {
    pub warehouse_id: Option<i64>,
    pub order_status: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
pub struct StockCountSummaryDto {
    pub id: i64,
    pub count_no: String,
    pub warehouse_id: i64,
    pub count_type: i32,
    pub plan_count_date: Option<chrono::NaiveDate>,
    pub actual_count_date: Option<chrono::NaiveDate>,
    pub order_status: i32,
}

#[derive(Debug, Deserialize)]
pub struct StockCountDetailPayload {
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub book_quantity: f64,
    pub counted_quantity: f64,
    pub unit: String,
}

#[derive(Debug, Deserialize)]
pub struct StockCountPayload {
    pub count_no: String,
    pub warehouse_id: i64,
    pub count_type: i32,
    pub plan_count_date: Option<chrono::NaiveDate>,
    pub remark: Option<String>,
    pub details: Vec<StockCountDetailPayload>,
}

#[derive(Debug, Serialize)]
pub struct StockCountDetailDto {
    pub id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub book_quantity: f64,
    pub counted_quantity: f64,
    pub diff_quantity: f64,
    pub unit: String,
    pub line_status: i32,
}

#[derive(Debug, Serialize)]
pub struct StockCountWithDetailsDto {
    pub header: StockCountSummaryDto,
    pub details: Vec<StockCountDetailDto>,
}



