use crate::db::entity::{self, stock_transactions, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

#[derive(Debug)]
pub struct StockTransactionFilter {
    pub material_id: Option<i64>,
    pub warehouse_id: Option<i64>,
    pub trans_type: Option<i32>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: StockTransactionFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<stock_transactions::Model>, u64)> {
    let mut query = stock_transactions::Entity::find()
        .filter(stock_transactions::Column::IsDeleted.eq(0));

    if let Some(m) = filter.material_id {
        query = query.filter(stock_transactions::Column::MaterialId.eq(m));
    }
    if let Some(w) = filter.warehouse_id {
        query = query.filter(stock_transactions::Column::WarehouseId.eq(w));
    }
    if let Some(t) = filter.trans_type {
        query = query.filter(stock_transactions::Column::TransType.eq(t));
    }

    query = query
        .order_by_desc(stock_transactions::Column::BusinessTime)
        .order_by_desc(stock_transactions::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}


