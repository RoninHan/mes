use crate::db::entity::{self, stock_adjustments, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug)]
pub struct StockAdjustmentFilter {
    pub warehouse_id: Option<i64>,
    pub material_id: Option<i64>,
    pub status: Option<i32>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: StockAdjustmentFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<stock_adjustments::Model>, u64)> {
    let mut query = stock_adjustments::Entity::find()
        .filter(stock_adjustments::Column::IsDeleted.eq(0));

    if let Some(w) = filter.warehouse_id {
        query = query.filter(stock_adjustments::Column::WarehouseId.eq(w));
    }
    if let Some(m) = filter.material_id {
        query = query.filter(stock_adjustments::Column::MaterialId.eq(m));
    }
    if let Some(s) = filter.status {
        query = query.filter(stock_adjustments::Column::Status.eq(s));
    }

    query = query.order_by_desc(stock_adjustments::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<stock_adjustments::Model>> {
    Ok(stock_adjustments::Entity::find_by_id(id)
        .filter(stock_adjustments::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: stock_adjustments::ActiveModel,
) -> Result<stock_adjustments::Model> {
    let res = stock_adjustments::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(stock_adjustments::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: stock_adjustments::ActiveModel,
) -> Result<Option<stock_adjustments::Model>> {
    active.id = Set(id);
    Ok(Some(
        stock_adjustments::Entity::update(active)
            .exec(conn)
            .await?,
    ))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = stock_adjustments::Entity::update_many()
        .col_expr(
            stock_adjustments::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(stock_adjustments::Column::Id.eq(id))
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}


