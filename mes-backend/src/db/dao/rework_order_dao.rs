use crate::db::entity::{self, rework_orders, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug, Default)]
pub struct ReworkOrderFilter {
    pub ncr_id: Option<i64>,
    pub rework_status: Option<i16>,
    pub material_id: Option<i64>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: ReworkOrderFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<rework_orders::Model>, u64)> {
    let mut query = rework_orders::Entity::find();

    if let Some(ncr_id) = filter.ncr_id {
        query = query.filter(rework_orders::Column::NcrId.eq(ncr_id));
    }
    if let Some(rework_status) = filter.rework_status {
        query = query.filter(rework_orders::Column::ReworkStatus.eq(rework_status));
    }
    if let Some(material_id) = filter.material_id {
        query = query.filter(rework_orders::Column::MaterialId.eq(material_id));
    }

    query = query
        .filter(rework_orders::Column::IsDeleted.eq(0))
        .order_by_desc(rework_orders::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<rework_orders::Model>> {
    Ok(rework_orders::Entity::find_by_id(id)
        .filter(rework_orders::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: rework_orders::ActiveModel,
) -> Result<rework_orders::Model> {
    Ok(rework_orders::Entity::insert(active)
        .exec(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: rework_orders::ActiveModel,
) -> Result<Option<rework_orders::Model>> {
    active.id = Set(id);
    Ok(Some(active.update(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active_model: rework_orders::ActiveModel = rework_orders::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Rework order not found"))?
        .into();
    
    active_model.is_deleted = Set(1);
    active_model.updated_time = Set(chrono::Utc::now().into());
    active_model.update(conn).await?;
    Ok(())
}


