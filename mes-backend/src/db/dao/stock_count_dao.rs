use crate::db::entity::{self, stock_count_lines, stock_count_orders, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

#[derive(Debug)]
pub struct StockCountFilter {
    pub warehouse_id: Option<i64>,
    pub order_status: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: StockCountFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<stock_count_orders::Model>, u64)> {
    let mut query = stock_count_orders::Entity::find()
        .filter(stock_count_orders::Column::IsDeleted.eq(0));

    if let Some(w) = filter.warehouse_id {
        query = query.filter(stock_count_orders::Column::WarehouseId.eq(w));
    }
    if let Some(s) = filter.order_status {
        query = query.filter(stock_count_orders::Column::OrderStatus.eq(s));
    }

    query = query.order_by_desc(stock_count_orders::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<(stock_count_orders::Model, Vec<stock_count_lines::Model>)>> {
    if let Some(order) = stock_count_orders::Entity::find_by_id(id)
        .filter(stock_count_orders::Column::IsDeleted.eq(0))
        .one(conn)
        .await?
    {
        let details = order
            .find_related(stock_count_lines::Entity)
            .filter(stock_count_lines::Column::IsDeleted.eq(0))
            .all(conn)
            .await?;
        Ok(Some((order, details)))
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub struct StockCountWithDetails {
    pub order: stock_count_orders::ActiveModel,
    pub details: Vec<stock_count_lines::ActiveModel>,
}

pub async fn create(
    conn: ConnRef<'_>,
    payload: StockCountWithDetails,
) -> Result<(stock_count_orders::Model, Vec<stock_count_lines::Model>)> {
    let txn = conn.begin().await?;

    let order = stock_count_orders::Entity::insert(payload.order)
        .exec_with_returning(&txn)
        .await?;

    let mut created_details = Vec::new();
    for mut d in payload.details {
        d.count_id = Set(order.id);
        let m = stock_count_lines::Entity::insert(d)
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
    payload: StockCountWithDetails,
) -> Result<Option<(stock_count_orders::Model, Vec<stock_count_lines::Model>)>> {
    let txn = conn.begin().await?;

    if stock_count_orders::Entity::find_by_id(id)
        .filter(stock_count_orders::Column::IsDeleted.eq(0))
        .one(&txn)
        .await?
        .is_none()
    {
        txn.rollback().await?;
        return Ok(None);
    }

    let mut order_active = payload.order;
    order_active.id = Set(id);
    let order = stock_count_orders::Entity::update(order_active)
        .exec_with_returning(&txn)
        .await?;

    stock_count_lines::Entity::update_many()
        .col_expr(stock_count_lines::Column::IsDeleted, sea_orm_migration::sea_query::Expr::value(1))
        .filter(stock_count_lines::Column::CountId.eq(id))
        .exec(&txn)
        .await?;

    let mut created_details = Vec::new();
    for mut d in payload.details {
        d.count_id = Set(order.id);
        let m = stock_count_lines::Entity::insert(d)
            .exec_with_returning(&txn)
            .await?;
        created_details.push(m);
    }

    txn.commit().await?;
    Ok((order, created_details))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let txn = conn.begin().await?;

    stock_count_lines::Entity::update_many()
        .col_expr(stock_count_lines::Column::IsDeleted, sea_orm_migration::sea_query::Expr::value(1))
        .filter(stock_count_lines::Column::CountId.eq(id))
        .exec(&txn)
        .await?;

    let res = stock_count_orders::Entity::update_many()
        .col_expr(
            stock_count_orders::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(stock_count_orders::Column::Id.eq(id))
        .exec(&txn)
        .await?;

    txn.commit().await?;
    Ok(res.rows_affected)
}


