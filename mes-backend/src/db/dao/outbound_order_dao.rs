use crate::db::entity::{self, outbound_order_lines, outbound_orders, ConnRef};
use anyhow::Result;
use sea_orm::{
    ModelTrait,
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

#[derive(Debug)]
pub struct OutboundFilter {
    pub warehouse_id: Option<i64>,
    pub outbound_type: Option<i32>,
    pub order_status: Option<i32>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: OutboundFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<outbound_orders::Model>, u64)> {
    let mut query = outbound_orders::Entity::find()
        .filter(outbound_orders::Column::IsDeleted.eq(0));

    if let Some(w) = filter.warehouse_id {
        query = query.filter(outbound_orders::Column::WarehouseId.eq(w));
    }
    if let Some(t) = filter.outbound_type {
        query = query.filter(outbound_orders::Column::OutboundType.eq(t));
    }
    if let Some(s) = filter.order_status {
        query = query.filter(outbound_orders::Column::OrderStatus.eq(s));
    }

    query = query.order_by_desc(outbound_orders::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<(outbound_orders::Model, Vec<outbound_order_lines::Model>)>> {
    if let Some(order) = outbound_orders::Entity::find_by_id(id)
        .filter(outbound_orders::Column::IsDeleted.eq(0))
        .one(conn)
        .await?
    {
        let details = outbound_order_lines::Entity::find()
            .filter(outbound_order_lines::Column::OutboundId.eq(order.id))
            .filter(outbound_order_lines::Column::IsDeleted.eq(0))
            .all(conn)
            .await?;
        Ok(Some((order, details)))
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OutboundWithDetails {
    pub order: outbound_orders::ActiveModel,
    pub details: Vec<outbound_order_lines::ActiveModel>,
}

pub async fn create(
    conn: ConnRef<'_>,
    payload: OutboundWithDetails,
) -> Result<(outbound_orders::Model, Vec<outbound_order_lines::Model>)> {
    let txn = conn.begin().await?;

    let order_result = outbound_orders::Entity::insert(payload.order)
        .exec(&txn)
        .await?;
    let order_id = order_result.last_insert_id;

    let mut created_detail_ids = Vec::new();
    for mut d in payload.details {
        d.outbound_id = Set(order_id);
        let m = outbound_order_lines::Entity::insert(d)
            .exec(&txn)
            .await?;
        created_detail_ids.push(m.last_insert_id);
    }

    txn.commit().await?;

    let order = outbound_orders::Entity::find_by_id(order_id)
        .one(conn)
        .await?
        .unwrap();
    let mut created_details = Vec::new();
    for id in created_detail_ids {
        if let Some(m) = outbound_order_lines::Entity::find_by_id(id).one(conn).await? {
            created_details.push(m);
        }
    }
    Ok((order, created_details))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    payload: OutboundWithDetails,
) -> Result<Option<(outbound_orders::Model, Vec<outbound_order_lines::Model>)>> {
    let txn = conn.begin().await?;

    if outbound_orders::Entity::find_by_id(id)
        .filter(outbound_orders::Column::IsDeleted.eq(0))
        .one(&txn)
        .await?
        .is_none()
    {
        txn.rollback().await?;
        return Ok(None);
    }

    let mut order_active = payload.order;
    order_active.id = Set(id);
    outbound_orders::Entity::update(order_active)
        .exec(&txn)
        .await?;

    outbound_order_lines::Entity::update_many()
        .col_expr(
            outbound_order_lines::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(outbound_order_lines::Column::OutboundId.eq(id))
        .exec(&txn)
        .await?;

    let mut created_detail_ids = Vec::new();
    for mut d in payload.details {
        d.outbound_id = Set(id);
        let m = outbound_order_lines::Entity::insert(d)
            .exec(&txn)
            .await?;
        created_detail_ids.push(m.last_insert_id);
    }

    txn.commit().await?;

    let order = outbound_orders::Entity::find_by_id(id)
        .one(conn)
        .await?
        .unwrap();
    let mut created_details = Vec::new();
    for detail_id in created_detail_ids {
        if let Some(m) = outbound_order_lines::Entity::find_by_id(detail_id).one(conn).await? {
            created_details.push(m);
        }
    }
    Ok(Some((order, created_details)))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let txn = conn.begin().await?;

    outbound_order_lines::Entity::update_many()
        .col_expr(
            outbound_order_lines::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(outbound_order_lines::Column::OutboundId.eq(id))
        .exec(&txn)
        .await?;

    let res = outbound_orders::Entity::update_many()
        .col_expr(outbound_orders::Column::IsDeleted, sea_orm_migration::sea_query::Expr::value(1))
        .filter(outbound_orders::Column::Id.eq(id))
        .exec(&txn)
        .await?;

    txn.commit().await?;
    Ok(res.rows_affected)
}


