use crate::db::entity::{self, transfer_order_lines, transfer_orders, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

#[derive(Debug)]
pub struct TransferFilter {
    pub from_warehouse_id: Option<i64>,
    pub to_warehouse_id: Option<i64>,
    pub order_status: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: TransferFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<transfer_orders::Model>, u64)> {
    let mut query = entity::TransferOrders::find()
        .filter(transfer_orders::Column::IsDeleted.eq(0));

    if let Some(w) = filter.from_warehouse_id {
        query = query.filter(transfer_orders::Column::FromWarehouseId.eq(w));
    }
    if let Some(w) = filter.to_warehouse_id {
        query = query.filter(transfer_orders::Column::ToWarehouseId.eq(w));
    }
    if let Some(s) = filter.order_status {
        query = query.filter(transfer_orders::Column::OrderStatus.eq(s));
    }

    query = query.order_by_desc(transfer_orders::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<(transfer_orders::Model, Vec<transfer_order_lines::Model>)>> {
    if let Some(order) = entity::TransferOrders::find_by_id(id)
        .filter(transfer_orders::Column::IsDeleted.eq(0))
        .one(conn)
        .await?
    {
        let details = order
            .find_related(entity::TransferOrderLines)
            .filter(transfer_order_lines::Column::IsDeleted.eq(0))
            .all(conn)
            .await?;
        Ok(Some((order, details)))
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub struct TransferWithDetails {
    pub order: transfer_orders::ActiveModel,
    pub details: Vec<transfer_order_lines::ActiveModel>,
}

pub async fn create(
    conn: ConnRef<'_>,
    payload: TransferWithDetails,
) -> Result<(transfer_orders::Model, Vec<transfer_order_lines::Model>)> {
    let txn = conn.begin().await?;

    let order = entity::TransferOrders::insert(payload.order)
        .exec_with_returning(&txn)
        .await?;

    let mut created_details = Vec::new();
    for mut d in payload.details {
        d.transfer_id = Set(order.id);
        let m = entity::TransferOrderLines::insert(d)
            .exec_with_returning(&txn)
            .await?;
        created_details.push(m);
    }

    txn.commit().await?;
    Ok((order, created_details))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    payload: TransferWithDetails,
) -> Result<Option<(transfer_orders::Model, Vec<transfer_order_lines::Model>)>> {
    let txn = conn.begin().await?;

    if entity::TransferOrders::find_by_id(id)
        .filter(transfer_orders::Column::IsDeleted.eq(0))
        .one(&txn)
        .await?
        .is_none()
    {
        txn.rollback().await?;
        return Ok(None);
    }

    let mut order_active = payload.order;
    order_active.id = Set(id);
    let order = entity::TransferOrders::update(order_active)
        .exec_with_returning(&txn)
        .await?;

    entity::TransferOrderLines::update_many()
        .col_expr(
            transfer_order_lines::Column::IsDeleted,
            sea_orm::Expr::value(1),
        )
        .filter(transfer_order_lines::Column::TransferId.eq(id))
        .exec(&txn)
        .await?;

    let mut created_details = Vec::new();
    for mut d in payload.details {
        d.transfer_id = Set(order.id);
        let m = entity::TransferOrderLines::insert(d)
            .exec_with_returning(&txn)
            .await?;
        created_details.push(m);
    }

    txn.commit().await?;
    Ok((order, created_details))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let txn = conn.begin().await?;

    entity::TransferOrderLines::update_many()
        .col_expr(
            transfer_order_lines::Column::IsDeleted,
            sea_orm::Expr::value(1),
        )
        .filter(transfer_order_lines::Column::TransferId.eq(id))
        .exec(&txn)
        .await?;

    let res = entity::TransferOrders::update_many()
        .col_expr(transfer_orders::Column::IsDeleted, sea_orm::Expr::value(1))
        .filter(transfer_orders::Column::Id.eq(id))
        .exec(&txn)
        .await?;

    txn.commit().await?;
    Ok(res.rows_affected)
}


