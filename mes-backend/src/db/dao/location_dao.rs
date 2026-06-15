use crate::db::entity::{self, locations, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug, Default)]
pub struct LocationFilter {
    pub warehouse_id: Option<i64>,
    pub status: Option<i32>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: LocationFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<locations::Model>, u64)> {
    let mut query = locations::Entity::find().filter(locations::Column::IsDeleted.eq(0));

    if let Some(warehouse_id) = filter.warehouse_id {
        query = query.filter(locations::Column::WarehouseId.eq(warehouse_id));
    }
    if let Some(status) = filter.status {
        query = query.filter(locations::Column::Status.eq(status));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        query = query.filter(
            locations::Column::LocationName
                .like(like.clone())
                .or(locations::Column::LocationCode.like(like)),
        );
    }

    query = query.order_by_desc(locations::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<locations::Model>> {
    Ok(locations::Entity::find_by_id(id)
        .filter(locations::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: locations::ActiveModel,
) -> Result<locations::Model> {
    let res = locations::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(locations::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: locations::ActiveModel,
) -> Result<Option<locations::Model>> {
    active.id = Set(id);
    Ok(Some(locations::Entity::update(active).exec(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active: locations::ActiveModel = locations::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Location not found"))?
        .into();

    active.is_deleted = Set(1);
    active.updated_time = Set(chrono::Utc::now().into());
    locations::Entity::update(active).exec(conn).await?;
    Ok(())
}



