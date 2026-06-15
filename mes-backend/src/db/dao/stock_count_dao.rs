use crate::db::entity::{self, stock_count_lines, stock_count_orders, ConnRef};
use anyhow::Result;
use sea_orm::{
    ModelTrait,
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

#[derive(Debug)]
pub struct StockCountFilter {
    pub warehouse_id: Option<i64>,
    pub order_status: Option<i32>,
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
        let details = stock_count_lines::Entity::find()
            .filter(stock_count_lines::Column::CountId.eq(order.id))
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

    let order_result = stock_count_orders::Entity::insert(payload.order)
        .exec(&txn)
        .await?;
    let order_id = order_result.last_insert_id;

    let mut created_detail_ids = Vec::new();
    for mut d in payload.details {
        d.count_id = Set(order_id);
        let m = stock_count_lines::Entity::insert(d)
            .exec(&txn)
            .await?;
        created_detail_ids.push(m.last_insert_id);
    }

    txn.commit().await?;

    let order = stock_count_orders::Entity::find_by_id(order_id)
        .one(conn)
        .await?
        .unwrap();
    let mut created_details = Vec::new();
    for id in created_detail_ids {
        if let Some(m) = stock_count_lines::Entity::find_by_id(id).one(conn).await? {
            created_details.push(m);
        }
    }
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
        stock_count_orders::Entity::update(order_active)
            .exec(&txn)
            .await?;

        stock_count_lines::Entity::update_many()
            .col_expr(
                stock_count_lines::Column::IsDeleted,
                sea_orm_migration::sea_query::Expr::value(1),
            )
            .filter(stock_count_lines::Column::CountId.eq(id))
            .exec(&txn)
            .await?;

        let mut created_detail_ids = Vec::new();
        for mut d in payload.details {
            d.count_id = Set(id);
            let m = stock_count_lines::Entity::insert(d)
                .exec(&txn)
                .await?;
            created_detail_ids.push(m.last_insert_id);
        }

        txn.commit().await?;

        let order = stock_count_orders::Entity::find_by_id(id)
            .one(conn)
            .await?
            .unwrap();
        let mut created_details = Vec::new();
        for detail_id in created_detail_ids {
            if let Some(m) = stock_count_lines::Entity::find_by_id(detail_id).one(conn).await? {
                created_details.push(m);
            }
        }
        Ok(Some((order, created_details)))
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


