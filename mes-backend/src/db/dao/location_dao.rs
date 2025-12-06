use crate::db::entity::{self, locations, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug, Default)]
pub struct LocationFilter {
    pub warehouse_id: Option<i64>,
    pub status: Option<i16>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: LocationFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<locations::Model>, u64)> {
    let mut query = entity::Locations::find().filter(locations::Column::IsDeleted.eq(0));

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
                .ilike(like.clone())
                .or(locations::Column::LocationCode.ilike(like)),
        );
    }

    query = query.order_by_desc(locations::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<locations::Model>> {
    Ok(entity::Locations::find_by_id(id)
        .filter(locations::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: locations::ActiveModel,
) -> Result<locations::Model> {
    Ok(entity::Locations::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: locations::ActiveModel,
) -> Result<Option<locations::Model>> {
    active.id = Set(id);
    Ok(Some(active.update(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active: locations::ActiveModel = entity::Locations::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Location not found"))?
        .into();

    active.is_deleted = Set(1);
    active.updated_time = Set(chrono::Utc::now().into());
    active.update(conn).await?;
    Ok(())
}



