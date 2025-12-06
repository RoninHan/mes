use crate::db::entity::{self, quality_costs, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug, Default)]
pub struct QualityCostFilter {
    pub cost_period: Option<String>,
    pub cost_category: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: QualityCostFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<quality_costs::Model>, u64)> {
    let mut query = entity::QualityCosts::find();

    if let Some(ref period) = filter.cost_period {
        query = query.filter(quality_costs::Column::CostPeriod.eq(period.clone()));
    }
    if let Some(cost_category) = filter.cost_category {
        query = query.filter(quality_costs::Column::CostCategory.eq(cost_category));
    }

    query = query
        .filter(quality_costs::Column::IsDeleted.eq(0))
        .order_by_desc(quality_costs::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<quality_costs::Model>> {
    Ok(entity::QualityCosts::find_by_id(id)
        .filter(quality_costs::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: quality_costs::ActiveModel,
) -> Result<quality_costs::Model> {
    Ok(entity::QualityCosts::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: quality_costs::ActiveModel,
) -> Result<Option<quality_costs::Model>> {
    active.id = Set(id);
    Ok(Some(active.update(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active_model: quality_costs::ActiveModel = entity::QualityCosts::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Cost not found"))?
        .into();
    
    active_model.is_deleted = Set(1);
    active_model.updated_time = Set(chrono::Utc::now().into());
    active_model.update(conn).await?;
    Ok(())
}


