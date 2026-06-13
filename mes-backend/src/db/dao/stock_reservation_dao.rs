use crate::db::entity::{self, stock_reservations, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug)]
pub struct StockReservationFilter {
    pub material_id: Option<i64>,
    pub warehouse_id: Option<i64>,
    pub status: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: StockReservationFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<stock_reservations::Model>, u64)> {
    let mut query = stock_reservations::Entity::find()
        .filter(stock_reservations::Column::IsDeleted.eq(0));

    if let Some(m) = filter.material_id {
        query = query.filter(stock_reservations::Column::MaterialId.eq(m));
    }
    if let Some(w) = filter.warehouse_id {
        query = query.filter(stock_reservations::Column::WarehouseId.eq(w));
    }
    if let Some(s) = filter.status {
        query = query.filter(stock_reservations::Column::Status.eq(s));
    }

    query = query.order_by_desc(stock_reservations::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<stock_reservations::Model>> {
    Ok(stock_reservations::Entity::find_by_id(id)
        .filter(stock_reservations::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: stock_reservations::ActiveModel,
) -> Result<stock_reservations::Model> {
    Ok(stock_reservations::Entity::insert(active)
        .exec(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: stock_reservations::ActiveModel,
) -> Result<Option<stock_reservations::Model>> {
    active.id = Set(id);
    Ok(Some(
        stock_reservations::Entity::update(active)
            .exec(conn)
            .await?,
    ))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = stock_reservations::Entity::update_many()
        .col_expr(
            stock_reservations::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(stock_reservations::Column::Id.eq(id))
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}


