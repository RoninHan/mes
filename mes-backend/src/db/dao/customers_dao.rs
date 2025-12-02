use crate::db::entity::{self, customers, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug)]
pub struct CustomerFilter {
    pub customer_type: Option<i8>,
    pub status: Option<i8>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: CustomerFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<customers::Model>, u64)> {
    let mut query = entity::Customers::find();

    if let Some(t) = filter.customer_type {
        query = query.filter(customers::Column::CustomerType.eq(t));
    }
    if let Some(status) = filter.status {
        query = query.filter(customers::Column::Status.eq(status));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        query = query.filter(
            customers::Column::CustomerName
                .ilike(like.clone())
                .or(customers::Column::CustomerCode.ilike(like)),
        );
    }

    query = query.order_by_desc(customers::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<customers::Model>> {
    Ok(entity::Customers::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: customers::ActiveModel,
) -> Result<customers::Model> {
    Ok(entity::Customers::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: customers::ActiveModel,
) -> Result<Option<customers::Model>> {
    active.id = Set(id);
    let updated = entity::Customers::update(active)
        .exec_with_returning(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = entity::Customers::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


