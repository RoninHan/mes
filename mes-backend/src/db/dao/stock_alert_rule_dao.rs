use crate::db::entity::{self, stock_alert_rules, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug)]
pub struct StockAlertFilter {
    pub material_id: Option<i64>,
    pub warehouse_id: Option<i64>,
    pub enabled: Option<i32>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: StockAlertFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<stock_alert_rules::Model>, u64)> {
    let mut query = stock_alert_rules::Entity::find()
        .filter(stock_alert_rules::Column::IsDeleted.eq(0));

    if let Some(m) = filter.material_id {
        query = query.filter(stock_alert_rules::Column::MaterialId.eq(m));
    }
    if let Some(w) = filter.warehouse_id {
        query = query.filter(stock_alert_rules::Column::WarehouseId.eq(w));
    }
    if let Some(e) = filter.enabled {
        query = query.filter(stock_alert_rules::Column::Enabled.eq(e));
    }

    query = query.order_by_desc(stock_alert_rules::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<stock_alert_rules::Model>> {
    Ok(stock_alert_rules::Entity::find_by_id(id)
        .filter(stock_alert_rules::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: stock_alert_rules::ActiveModel,
) -> Result<stock_alert_rules::Model> {
    let res = stock_alert_rules::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(stock_alert_rules::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: stock_alert_rules::ActiveModel,
) -> Result<Option<stock_alert_rules::Model>> {
    active.id = Set(id);
    Ok(Some(
        stock_alert_rules::Entity::update(active)
            .exec(conn)
            .await?,
    ))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = stock_alert_rules::Entity::update_many()
        .col_expr(
            stock_alert_rules::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(stock_alert_rules::Column::Id.eq(id))
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}


