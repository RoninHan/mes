use crate::db::entity::{self, inbound_order_details, inbound_orders, ConnRef};
use anyhow::Result;
use sea_orm::{
    ModelTrait,
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

#[derive(Debug)]
pub struct InboundFilter {
    pub warehouse_id: Option<i64>,
    pub inbound_type: Option<i32>,
    pub order_status: Option<i32>,
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
        let details = inbound_order_details::Entity::find()
            .filter(inbound_order_details::Column::InboundOrderId.eq(order.id))
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

    let order_result = inbound_orders::Entity::insert(payload.order)
        .exec(&txn)
        .await?;
    let order_id = order_result.last_insert_id;

    let mut created_detail_ids = Vec::new();
    for mut d in payload.details {
        d.inbound_order_id = Set(order_id);
        let m = inbound_order_details::Entity::insert(d)
            .exec(&txn)
            .await?;
        created_detail_ids.push(m.last_insert_id);
    }

    txn.commit().await?;

    let order = inbound_orders::Entity::find_by_id(order_id)
        .one(conn)
        .await?
        .unwrap();
    let mut created_details = Vec::new();
    for id in created_detail_ids {
        if let Some(m) = inbound_order_details::Entity::find_by_id(id).one(conn).await? {
            created_details.push(m);
        }
    }
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
        inbound_orders::Entity::update(order_active)
            .exec(&txn)
            .await?;

        inbound_order_details::Entity::update_many()
            .col_expr(
                inbound_order_details::Column::IsDeleted,
                sea_orm_migration::sea_query::Expr::value(1),
            )
            .filter(inbound_order_details::Column::InboundOrderId.eq(id))
            .exec(&txn)
            .await?;

        let mut created_detail_ids = Vec::new();
        for mut d in payload.details {
            d.inbound_order_id = Set(id);
            let m = inbound_order_details::Entity::insert(d)
                .exec(&txn)
                .await?;
            created_detail_ids.push(m.last_insert_id);
        }

        txn.commit().await?;

        let order = inbound_orders::Entity::find_by_id(id)
            .one(conn)
            .await?
            .unwrap();
        let mut created_details = Vec::new();
        for detail_id in created_detail_ids {
            if let Some(m) = inbound_order_details::Entity::find_by_id(detail_id).one(conn).await? {
                created_details.push(m);
            }
        }
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


