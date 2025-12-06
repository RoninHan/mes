use crate::db::entity::{self, process_routes, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug, Default)]
pub struct ProcessRouteFilter {
    pub material_id: Option<i64>,
    pub status: Option<i16>,
    pub is_default: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: ProcessRouteFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<process_routes::Model>, u64)> {
    let mut query = entity::ProcessRoutes::find().filter(process_routes::Column::IsDeleted.eq(0));

    if let Some(material_id) = filter.material_id {
        query = query.filter(process_routes::Column::MaterialId.eq(material_id));
    }
    if let Some(status) = filter.status {
        query = query.filter(process_routes::Column::Status.eq(status));
    }
    if let Some(is_default) = filter.is_default {
        query = query.filter(process_routes::Column::IsDefault.eq(is_default));
    }

    query = query.order_by_desc(process_routes::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<process_routes::Model>> {
    Ok(entity::ProcessRoutes::find_by_id(id)
        .filter(process_routes::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: process_routes::ActiveModel,
) -> Result<process_routes::Model> {
    Ok(entity::ProcessRoutes::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: process_routes::ActiveModel,
) -> Result<Option<process_routes::Model>> {
    active.id = Set(id);
    Ok(Some(active.update(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active: process_routes::ActiveModel = entity::ProcessRoutes::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Process route not found"))?
        .into();

    active.is_deleted = Set(1);
    active.updated_time = Set(chrono::Utc::now().into());
    active.update(conn).await?;
    Ok(())
}



