use crate::db::entity::{self, quality_kpi, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug, Default)]
pub struct QualityKpiFilter {
    pub kpi_type: Option<i32>,
    pub dept_id: Option<i64>,
    pub workshop_id: Option<i64>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: QualityKpiFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<quality_kpi::Model>, u64)> {
    let mut query = quality_kpi::Entity::find();

    if let Some(kpi_type) = filter.kpi_type {
        query = query.filter(quality_kpi::Column::KpiType.eq(kpi_type));
    }
    if let Some(dept_id) = filter.dept_id {
        query = query.filter(quality_kpi::Column::DeptId.eq(dept_id));
    }
    if let Some(workshop_id) = filter.workshop_id {
        query = query.filter(quality_kpi::Column::WorkshopId.eq(workshop_id));
    }
    if let Some(start_date) = filter.start_date {
        query = query.filter(quality_kpi::Column::KpiDate.gte(start_date));
    }
    if let Some(end_date) = filter.end_date {
        query = query.filter(quality_kpi::Column::KpiDate.lte(end_date));
    }

    query = query.order_by_desc(quality_kpi::Column::KpiDate);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<quality_kpi::Model>> {
    Ok(quality_kpi::Entity::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: quality_kpi::ActiveModel,
) -> Result<quality_kpi::Model> {
    let res = quality_kpi::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(quality_kpi::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: quality_kpi::ActiveModel,
) -> Result<Option<quality_kpi::Model>> {
    active.id = Set(id);
    Ok(Some(quality_kpi::Entity::update(active).exec(conn).await?))
}


