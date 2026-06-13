use crate::db::entity::{self, supplier_quality_evaluations, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug, Default)]
pub struct SupplierQualityEvaluationFilter {
    pub supplier_id: Option<i64>,
    pub evaluation_period: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: SupplierQualityEvaluationFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<supplier_quality_evaluations::Model>, u64)> {
    let mut query = supplier_quality_evaluations::Entity::find();

    if let Some(supplier_id) = filter.supplier_id {
        query = query.filter(supplier_quality_evaluations::Column::SupplierId.eq(supplier_id));
    }
    if let Some(ref period) = filter.evaluation_period {
        query = query.filter(supplier_quality_evaluations::Column::EvaluationPeriod.eq(period.clone()));
    }

    query = query
        .filter(supplier_quality_evaluations::Column::IsDeleted.eq(0))
        .order_by_desc(supplier_quality_evaluations::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<supplier_quality_evaluations::Model>> {
    Ok(supplier_quality_evaluations::Entity::find_by_id(id)
        .filter(supplier_quality_evaluations::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: supplier_quality_evaluations::ActiveModel,
) -> Result<supplier_quality_evaluations::Model> {
    Ok(supplier_quality_evaluations::Entity::insert(active)
        .exec(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: supplier_quality_evaluations::ActiveModel,
) -> Result<Option<supplier_quality_evaluations::Model>> {
    active.id = Set(id);
    Ok(Some(active.update(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active_model: supplier_quality_evaluations::ActiveModel = supplier_quality_evaluations::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Evaluation not found"))?
        .into();
    
    active_model.is_deleted = Set(1);
    active_model.updated_time = Set(chrono::Utc::now().into());
    active_model.update(conn).await?;
    Ok(())
}


