use crate::db::entity::{self, production_orders, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug)]
pub struct OrderFilter {
    pub order_status: Option<i32>,
    pub workshop_id: Option<i64>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: OrderFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<production_orders::Model>, u64)> {
    let mut query = production_orders::Entity::find();

    if let Some(status) = filter.order_status {
        query = query.filter(production_orders::Column::OrderStatus.eq(status));
    }
    if let Some(workshop) = filter.workshop_id {
        query = query.filter(production_orders::Column::WorkshopId.eq(workshop));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        query = query.filter(
            production_orders::Column::OrderNo
                .like(like.clone())
                .or(production_orders::Column::BatchNo.like(like)),
        );
    }

    query = query.order_by_desc(production_orders::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<production_orders::Model>> {
    Ok(production_orders::Entity::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: production_orders::ActiveModel,
) -> Result<production_orders::Model> {
    let res = production_orders::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(production_orders::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: production_orders::ActiveModel,
) -> Result<Option<production_orders::Model>> {
    active.id = Set(id);
    let updated = production_orders::Entity::update(active)
        .exec(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = production_orders::Entity::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


