use crate::api::ApiContext;
use crate::db::dao;
use crate::model::warehouse::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_orm::ActiveValue::Set;

pub fn router() -> axum::Router<ApiContext> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/warehouse/inventory", get(list_inventory))
        .route("/warehouse/inventory/:id", get(get_inventory))
        .route(
            "/warehouse/inbound-orders",
            get(list_inbound_orders).post(create_inbound_order),
        )
        .route(
            "/warehouse/inbound-orders/:id",
            get(get_inbound_order)
                .put(update_inbound_order)
                .delete(delete_inbound_order),
        )
        .route(
            "/warehouse/outbound-orders",
            get(list_outbound_orders).post(create_outbound_order),
        )
        .route(
            "/warehouse/outbound-orders/:id",
            get(get_outbound_order)
                .put(update_outbound_order)
                .delete(delete_outbound_order),
        )
        .route(
            "/warehouse/transfer-orders",
            get(list_transfer_orders).post(create_transfer_order),
        )
        .route(
            "/warehouse/transfer-orders/:id",
            get(get_transfer_order)
                .put(update_transfer_order)
                .delete(delete_transfer_order),
        )
        .route(
            "/warehouse/stock-count-orders",
            get(list_stock_count_orders).post(create_stock_count_order),
        )
        .route(
            "/warehouse/stock-count-orders/:id",
            get(get_stock_count_order)
                .put(update_stock_count_order)
                .delete(delete_stock_count_order),
        )
}

async fn list_inventory(
    State(ctx): State<ApiContext>,
    Query(q): Query<InventoryQuery>,
) -> Result<Json<PageResult<InventoryDto>>, StatusCode> {
    let filter = dao::inventory_dao::InventoryFilter {
        material_id: q.material_id,
        warehouse_id: q.warehouse_id,
        location_id: q.location_id,
        batch_no: q.batch_no.clone(),
    };
    let (items, total) =
        dao::inventory_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| InventoryDto {
            id: m.id,
            material_id: m.material_id,
            warehouse_id: m.warehouse_id,
            location_id: m.location_id,
            batch_no: m.batch_no,
            serial_no: m.serial_no,
            quantity: m.quantity.to_f64().unwrap_or(0.0),
            available_quantity: m.available_quantity.to_f64().unwrap_or(0.0),
            locked_quantity: m.locked_quantity.to_f64().unwrap_or(0.0),
            unit: m.unit,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_inventory(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<InventoryDto>, StatusCode> {
    let Some(m) = dao::inventory_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(InventoryDto {
        id: m.id,
        material_id: m.material_id,
        warehouse_id: m.warehouse_id,
        location_id: m.location_id,
        batch_no: m.batch_no,
        serial_no: m.serial_no,
        quantity: m.quantity.to_f64().unwrap_or(0.0),
        available_quantity: m.available_quantity.to_f64().unwrap_or(0.0),
        locked_quantity: m.locked_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
    }))
}

async fn list_inbound_orders(
    State(ctx): State<ApiContext>,
    Query(q): Query<InboundQuery>,
) -> Result<Json<PageResult<InboundSummaryDto>>, StatusCode> {
    let filter = dao::inbound_order_dao::InboundFilter {
        warehouse_id: q.warehouse_id,
        inbound_type: q.inbound_type,
        order_status: q.order_status,
    };
    let (items, total) =
        dao::inbound_order_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| InboundSummaryDto {
            id: m.id,
            inbound_no: m.inbound_no,
            inbound_type: m.inbound_type,
            warehouse_id: m.warehouse_id,
            supplier_id: m.supplier_id,
            plan_inbound_date: m.plan_inbound_date,
            actual_inbound_date: m.actual_inbound_date,
            total_quantity: m.total_quantity.to_f64().unwrap_or(0.0),
            order_status: m.order_status,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_inbound_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<InboundWithDetailsDto>, StatusCode> {
    let Some((h, ds)) = dao::inbound_order_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let header = InboundSummaryDto {
        id: h.id,
        inbound_no: h.inbound_no,
        inbound_type: h.inbound_type,
        warehouse_id: h.warehouse_id,
        supplier_id: h.supplier_id,
        plan_inbound_date: h.plan_inbound_date,
        actual_inbound_date: h.actual_inbound_date,
        total_quantity: h.total_quantity.to_f64().unwrap_or(0.0),
        order_status: h.order_status,
    };

    let details = ds
        .into_iter()
        .map(|d| InboundDetailDto {
            id: d.id,
            material_id: d.material_id,
            location_id: d.location_id,
            batch_no: d.batch_no,
            plan_quantity: d.plan_quantity.to_f64().unwrap_or(0.0),
            actual_quantity: d.actual_quantity.to_f64().unwrap_or(0.0),
            unit: d.unit,
            unit_price: d.unit_price.to_f64().unwrap_or(0.0),
            amount: d.amount.to_f64().unwrap_or(0.0),
        })
        .collect();

    Ok(Json(InboundWithDetailsDto { header, details }))
}

async fn create_inbound_order(
    State(ctx): State<ApiContext>,
    Json(body): Json<InboundPayload>,
) -> Result<Json<InboundWithDetailsDto>, StatusCode> {
    let order_active = crate::db::entity::inbound_orders::ActiveModel {
        inbound_no: Set(body.inbound_no),
        inbound_type: Set(body.inbound_type),
        warehouse_id: Set(body.warehouse_id),
        supplier_id: Set(body.supplier_id),
        plan_inbound_date: Set(body.plan_inbound_date),
        order_status: Set(1), // 待入库
        remark: Set(body.remark),
        total_quantity: Set(Decimal::from_f64(
            body.details.iter().map(|d| d.plan_quantity).sum(),
        )
        .unwrap_or_default()),
        ..Default::default()
    };

    let details_active = body
        .details
        .into_iter()
        .map(|d| {
            let qty = Decimal::from_f64(d.plan_quantity).unwrap_or_default();
            let price = Decimal::from_f64(d.unit_price).unwrap_or_default();
            crate::db::entity::inbound_order_details::ActiveModel {
                material_id: Set(d.material_id),
                location_id: Set(d.location_id),
                batch_no: Set(d.batch_no),
                plan_quantity: Set(qty),
                actual_quantity: Set(Decimal::ZERO),
                qualified_quantity: Set(Decimal::ZERO),
                unqualified_quantity: Set(Decimal::ZERO),
                unit: Set(d.unit),
                unit_price: Set(price),
                amount: Set(qty * price),
                quality_status: Set(1),
                line_status: Set(1),
                ..Default::default()
            }
        })
        .collect();

    let (h, ds) = dao::inbound_order_dao::create(
        ctx.db.conn(),
        dao::inbound_order_dao::InboundWithDetails {
            order: order_active,
            details: details_active,
        },
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let header = InboundSummaryDto {
        id: h.id,
        inbound_no: h.inbound_no,
        inbound_type: h.inbound_type,
        warehouse_id: h.warehouse_id,
        supplier_id: h.supplier_id,
        plan_inbound_date: h.plan_inbound_date,
        actual_inbound_date: h.actual_inbound_date,
        total_quantity: h.total_quantity.to_f64().unwrap_or(0.0),
        order_status: h.order_status,
    };

    let details = ds
        .into_iter()
        .map(|d| InboundDetailDto {
            id: d.id,
            material_id: d.material_id,
            location_id: d.location_id,
            batch_no: d.batch_no,
            plan_quantity: d.plan_quantity.to_f64().unwrap_or(0.0),
            actual_quantity: d.actual_quantity.to_f64().unwrap_or(0.0),
            unit: d.unit,
            unit_price: d.unit_price.to_f64().unwrap_or(0.0),
            amount: d.amount.to_f64().unwrap_or(0.0),
        })
        .collect();

    Ok(Json(InboundWithDetailsDto { header, details }))
}

async fn update_inbound_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<InboundPayload>,
) -> Result<Json<InboundWithDetailsDto>, StatusCode> {
    let order_active = crate::db::entity::inbound_orders::ActiveModel {
        inbound_no: Set(body.inbound_no),
        inbound_type: Set(body.inbound_type),
        warehouse_id: Set(body.warehouse_id),
        supplier_id: Set(body.supplier_id),
        plan_inbound_date: Set(body.plan_inbound_date),
        remark: Set(body.remark),
        total_quantity: Set(Decimal::from_f64(
            body.details.iter().map(|d| d.plan_quantity).sum(),
        )
        .unwrap_or_default()),
        ..Default::default()
    };

    let details_active = body
        .details
        .into_iter()
        .map(|d| {
            let qty = Decimal::from_f64(d.plan_quantity).unwrap_or_default();
            let price = Decimal::from_f64(d.unit_price).unwrap_or_default();
            crate::db::entity::inbound_order_details::ActiveModel {
                material_id: Set(d.material_id),
                location_id: Set(d.location_id),
                batch_no: Set(d.batch_no),
                plan_quantity: Set(qty),
                actual_quantity: Set(Decimal::ZERO),
                qualified_quantity: Set(Decimal::ZERO),
                unqualified_quantity: Set(Decimal::ZERO),
                unit: Set(d.unit),
                unit_price: Set(price),
                amount: Set(qty * price),
                quality_status: Set(1),
                line_status: Set(1),
                ..Default::default()
            }
        })
        .collect();

    let Some((h, ds)) = dao::inbound_order_dao::update(
        ctx.db.conn(),
        id,
        dao::inbound_order_dao::InboundWithDetails {
            order: order_active,
            details: details_active,
        },
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let header = InboundSummaryDto {
        id: h.id,
        inbound_no: h.inbound_no,
        inbound_type: h.inbound_type,
        warehouse_id: h.warehouse_id,
        supplier_id: h.supplier_id,
        plan_inbound_date: h.plan_inbound_date,
        actual_inbound_date: h.actual_inbound_date,
        total_quantity: h.total_quantity.to_f64().unwrap_or(0.0),
        order_status: h.order_status,
    };

    let details = ds
        .into_iter()
        .map(|d| InboundDetailDto {
            id: d.id,
            material_id: d.material_id,
            location_id: d.location_id,
            batch_no: d.batch_no,
            plan_quantity: d.plan_quantity.to_f64().unwrap_or(0.0),
            actual_quantity: d.actual_quantity.to_f64().unwrap_or(0.0),
            unit: d.unit,
            unit_price: d.unit_price.to_f64().unwrap_or(0.0),
            amount: d.amount.to_f64().unwrap_or(0.0),
        })
        .collect();

    Ok(Json(InboundWithDetailsDto { header, details }))
}

async fn delete_inbound_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::inbound_order_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}


