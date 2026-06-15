use crate::db::entity::{self, suppliers, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug)]
pub struct SupplierFilter {
    pub supplier_type: Option<i32>,
    pub status: Option<i32>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: SupplierFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<suppliers::Model>, u64)> {
    let mut query = suppliers::Entity::find();

    if let Some(t) = filter.supplier_type {
        query = query.filter(suppliers::Column::SupplierType.eq(t));
    }
    if let Some(status) = filter.status {
        query = query.filter(suppliers::Column::Status.eq(status));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        query = query.filter(
            suppliers::Column::SupplierName
                .like(like.clone())
                .or(suppliers::Column::SupplierCode.like(like)),
        );
    }

    query = query.order_by_desc(suppliers::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<suppliers::Model>> {
    Ok(suppliers::Entity::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: suppliers::ActiveModel,
) -> Result<suppliers::Model> {
    let res = suppliers::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(suppliers::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: suppliers::ActiveModel,
) -> Result<Option<suppliers::Model>> {
    active.id = Set(id);
    let updated = suppliers::Entity::update(active)
        .exec(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = suppliers::Entity::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


