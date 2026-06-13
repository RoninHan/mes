use crate::db::entity::{self, picking_order_lines, picking_orders, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

#[derive(Debug)]
pub struct PickingFilter {
    pub production_order_id: Option<i64>,
    pub warehouse_id: Option<i64>,
    pub order_status: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: PickingFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<picking_orders::Model>, u64)> {
    let mut query = picking_orders::Entity::find()
        .filter(picking_orders::Column::IsDeleted.eq(0));

    if let Some(po) = filter.production_order_id {
        query = query.filter(picking_orders::Column::ProductionOrderId.eq(po));
    }
    if let Some(w) = filter.warehouse_id {
        query = query.filter(picking_orders::Column::WarehouseId.eq(w));
    }
    if let Some(s) = filter.order_status {
        query = query.filter(picking_orders::Column::OrderStatus.eq(s));
    }

    query = query.order_by_desc(picking_orders::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<(picking_orders::Model, Vec<picking_order_lines::Model>)>> {
    if let Some(order) = picking_orders::Entity::find_by_id(id)
        .filter(picking_orders::Column::IsDeleted.eq(0))
        .one(conn)
        .await?
    {
        let details = order
            .find_related(picking_order_lines::Entity)
            .filter(picking_order_lines::Column::IsDeleted.eq(0))
            .all(conn)
            .await?;
        Ok(Some((order, details)))
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub struct PickingWithDetails {
    pub order: picking_orders::ActiveModel,
    pub details: Vec<picking_order_lines::ActiveModel>,
}

pub async fn create(
    conn: ConnRef<'_>,
    payload: PickingWithDetails,
) -> Result<(picking_orders::Model, Vec<picking_order_lines::Model>)> {
    let txn = conn.begin().await?;

    let order = picking_orders::Entity::insert(payload.order)
        .exec_with_returning(&txn)
        .await?;

    let mut created_details = Vec::new();
    for mut d in payload.details {
        d.picking_id = Set(order.id);
        let m = picking_order_lines::Entity::insert(d)
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
    payload: PickingWithDetails,
) -> Result<Option<(picking_orders::Model, Vec<picking_order_lines::Model>)>> {
    let txn = conn.begin().await?;

    if picking_orders::Entity::find_by_id(id)
        .filter(picking_orders::Column::IsDeleted.eq(0))
        .one(&txn)
        .await?
        .is_none()
    {
        txn.rollback().await?;
        return Ok(None);
    }

    let mut order_active = payload.order;
    order_active.id = Set(id);
    let order = picking_orders::Entity::update(order_active)
        .exec_with_returning(&txn)
        .await?;

    picking_order_lines::Entity::update_many()
        .col_expr(
            picking_order_lines::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(picking_order_lines::Column::PickingId.eq(id))
        .exec(&txn)
        .await?;

    let mut created_details = Vec::new();
    for mut d in payload.details {
        d.picking_id = Set(order.id);
        let m = picking_order_lines::Entity::insert(d)
            .exec_with_returning(&txn)
            .await?;
        created_details.push(m);
    }

    txn.commit().await?;
    Ok((order, created_details))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let txn = conn.begin().await?;

    picking_order_lines::Entity::update_many()
        .col_expr(
            picking_order_lines::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(picking_order_lines::Column::PickingId.eq(id))
        .exec(&txn)
        .await?;

    let res = picking_orders::Entity::update_many()
        .col_expr(picking_orders::Column::IsDeleted, sea_orm_migration::sea_query::Expr::value(1))
        .filter(picking_orders::Column::Id.eq(id))
        .exec(&txn)
        .await?;

    txn.commit().await?;
    Ok(res.rows_affected)
}


