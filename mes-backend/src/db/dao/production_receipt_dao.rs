use crate::db::entity::{self, production_receipts, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug)]
pub struct ProductionReceiptFilter {
    pub production_order_id: Option<i64>,
    pub warehouse_id: Option<i64>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: ProductionReceiptFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<production_receipts::Model>, u64)> {
    let mut query = production_receipts::Entity::find()
        .filter(production_receipts::Column::IsDeleted.eq(0));

    if let Some(po) = filter.production_order_id {
        query = query.filter(production_receipts::Column::ProductionOrderId.eq(po));
    }
    if let Some(w) = filter.warehouse_id {
        query = query.filter(production_receipts::Column::WarehouseId.eq(w));
    }

    query = query.order_by_desc(production_receipts::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<production_receipts::Model>> {
    Ok(production_receipts::Entity::find_by_id(id)
        .filter(production_receipts::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: production_receipts::ActiveModel,
) -> Result<production_receipts::Model> {
    Ok(production_receipts::Entity::insert(active)
        .exec(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: production_receipts::ActiveModel,
) -> Result<Option<production_receipts::Model>> {
    active.id = Set(id);
    Ok(Some(
        production_receipts::Entity::update(active)
            .exec(conn)
            .await?,
    ))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = production_receipts::Entity::update_many()
        .col_expr(
            production_receipts::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(production_receipts::Column::Id.eq(id))
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}


