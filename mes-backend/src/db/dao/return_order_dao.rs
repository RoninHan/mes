use crate::db::entity::{self, return_order_lines, return_orders, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

#[derive(Debug)]
pub struct ReturnFilter {
    pub production_order_id: Option<i64>,
    pub warehouse_id: Option<i64>,
    pub order_status: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: ReturnFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<return_orders::Model>, u64)> {
    let mut query = return_orders::Entity::find()
        .filter(return_orders::Column::IsDeleted.eq(0));

    if let Some(po) = filter.production_order_id {
        query = query.filter(return_orders::Column::ProductionOrderId.eq(po));
    }
    if let Some(w) = filter.warehouse_id {
        query = query.filter(return_orders::Column::WarehouseId.eq(w));
    }
    if let Some(s) = filter.order_status {
        query = query.filter(return_orders::Column::OrderStatus.eq(s));
    }

    query = query.order_by_desc(return_orders::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<(return_orders::Model, Vec<return_order_lines::Model>)>> {
    if let Some(order) = return_orders::Entity::find_by_id(id)
        .filter(return_orders::Column::IsDeleted.eq(0))
        .one(conn)
        .await?
    {
        let details = order
            .find_related(return_order_lines::Entity)
            .filter(return_order_lines::Column::IsDeleted.eq(0))
            .all(conn)
            .await?;
        Ok(Some((order, details)))
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub struct ReturnWithDetails {
    pub order: return_orders::ActiveModel,
    pub details: Vec<return_order_lines::ActiveModel>,
}

pub async fn create(
    conn: ConnRef<'_>,
    payload: ReturnWithDetails,
) -> Result<(return_orders::Model, Vec<return_order_lines::Model>)> {
    let txn = conn.begin().await?;

    let order = return_orders::Entity::insert(payload.order)
        .exec_with_returning(&txn)
        .await?;

    let mut created_details = Vec::new();
    for mut d in payload.details {
        d.return_id = Set(order.id);
        let m = return_order_lines::Entity::insert(d)
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
    payload: ReturnWithDetails,
) -> Result<Option<(return_orders::Model, Vec<return_order_lines::Model>)>> {
    let txn = conn.begin().await?;

    if return_orders::Entity::find_by_id(id)
        .filter(return_orders::Column::IsDeleted.eq(0))
        .one(&txn)
        .await?
        .is_none()
    {
        txn.rollback().await?;
        return Ok(None);
    }

    let mut order_active = payload.order;
    order_active.id = Set(id);
    let order = return_orders::Entity::update(order_active)
        .exec_with_returning(&txn)
        .await?;

    return_order_lines::Entity::update_many()
        .col_expr(
            return_order_lines::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(return_order_lines::Column::ReturnId.eq(id))
        .exec(&txn)
        .await?;

    let mut created_details = Vec::new();
    for mut d in payload.details {
        d.return_id = Set(order.id);
        let m = return_order_lines::Entity::insert(d)
            .exec_with_returning(&txn)
            .await?;
        created_details.push(m);
    }

    txn.commit().await?;
    Ok((order, created_details))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let txn = conn.begin().await?;

    return_order_lines::Entity::update_many()
        .col_expr(
            return_order_lines::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(return_order_lines::Column::ReturnId.eq(id))
        .exec(&txn)
        .await?;

    let res = return_orders::Entity::update_many()
        .col_expr(return_orders::Column::IsDeleted, sea_orm_migration::sea_query::Expr::value(1))
        .filter(return_orders::Column::Id.eq(id))
        .exec(&txn)
        .await?;

    txn.commit().await?;
    Ok(res.rows_affected)
}


