use crate::db::entity::{self, suppliers, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug)]
pub struct SupplierFilter {
    pub supplier_type: Option<i8>,
    pub status: Option<i8>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: SupplierFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<suppliers::Model>, u64)> {
    let mut query = entity::Suppliers::find();

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
                .ilike(like.clone())
                .or(suppliers::Column::SupplierCode.ilike(like)),
        );
    }

    query = query.order_by_desc(suppliers::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<suppliers::Model>> {
    Ok(entity::Suppliers::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: suppliers::ActiveModel,
) -> Result<suppliers::Model> {
    Ok(entity::Suppliers::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: suppliers::ActiveModel,
) -> Result<Option<suppliers::Model>> {
    active.id = Set(id);
    let updated = entity::Suppliers::update(active)
        .exec_with_returning(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = entity::Suppliers::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


