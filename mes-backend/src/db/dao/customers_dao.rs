use crate::db::entity::{self, customers, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug)]
pub struct CustomerFilter {
    pub customer_type: Option<i32>,
    pub status: Option<i32>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: CustomerFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<customers::Model>, u64)> {
    let mut query = customers::Entity::find();

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
                .like(like.clone())
                .or(customers::Column::CustomerCode.like(like)),
        );
    }

    query = query.order_by_desc(customers::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<customers::Model>> {
    Ok(customers::Entity::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: customers::ActiveModel,
) -> Result<customers::Model> {
    let res = customers::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(customers::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: customers::ActiveModel,
) -> Result<Option<customers::Model>> {
    active.id = Set(id);
    let updated = customers::Entity::update(active)
        .exec(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = customers::Entity::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


