use crate::db::entity::{self, inbound_order_details, inbound_orders, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

#[derive(Debug)]
pub struct InboundFilter {
    pub warehouse_id: Option<i64>,
    pub inbound_type: Option<i8>,
    pub order_status: Option<i8>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: InboundFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<inbound_orders::Model>, u64)> {
    let mut query = inbound_orders::Entity::find();

    if let Some(w) = filter.warehouse_id {
        query = query.filter(inbound_orders::Column::WarehouseId.eq(w));
    }
    if let Some(t) = filter.inbound_type {
        query = query.filter(inbound_orders::Column::InboundType.eq(t));
    }
    if let Some(s) = filter.order_status {
        query = query.filter(inbound_orders::Column::OrderStatus.eq(s));
    }

    query = query.order_by_desc(inbound_orders::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<(inbound_orders::Model, Vec<inbound_order_details::Model>)>> {
    if let Some(order) = inbound_orders::Entity::find_by_id(id).one(conn).await? {
        let details = order
            .find_related(inbound_order_details::Entity)
            .all(conn)
            .await?;
        Ok(Some((order, details)))
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub struct InboundWithDetails {
    pub order: inbound_orders::ActiveModel,
    pub details: Vec<inbound_order_details::ActiveModel>,
}

pub async fn create(
    conn: ConnRef<'_>,
    payload: InboundWithDetails,
) -> Result<(inbound_orders::Model, Vec<inbound_order_details::Model>)> {
    let txn = conn.begin().await?;

    let order = inbound_orders::Entity::insert(payload.order)
        .exec_with_returning(&txn)
        .await?;

    let mut created_details = Vec::new();
    for mut d in payload.details {
        d.inbound_order_id = Set(order.id);
        let m = inbound_order_details::Entity::insert(d)
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
    payload: InboundWithDetails,
) -> Result<Option<(inbound_orders::Model, Vec<inbound_order_details::Model>)>> {
    let txn = conn.begin().await?;

    // ensure exists
    if inbound_orders::Entity::find_by_id(id).one(&txn).await?.is_none() {
        txn.rollback().await?;
        return Ok(None);
    }

    let mut order_active = payload.order;
    order_active.id = Set(id);
    let order = inbound_orders::Entity::update(order_active)
        .exec_with_returning(&txn)
        .await?;

    // simple strategy: delete all details and re-insert
    inbound_order_details::Entity::delete_many()
        .filter(inbound_order_details::Column::InboundOrderId.eq(id))
        .exec(&txn)
        .await?;

    let mut created_details = Vec::new();
    for mut d in payload.details {
        d.inbound_order_id = Set(order.id);
        let m = inbound_order_details::Entity::insert(d)
            .exec_with_returning(&txn)
            .await?;
        created_details.push(m);
    }

    txn.commit().await?;
    Ok(Some((order, created_details)))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let txn = conn.begin().await?;

    inbound_order_details::Entity::delete_many()
        .filter(inbound_order_details::Column::InboundOrderId.eq(id))
        .exec(&txn)
        .await?;

    let res = inbound_orders::Entity::delete_by_id(id).exec(&txn).await?;
    txn.commit().await?;
    Ok(res.rows_affected)
}


