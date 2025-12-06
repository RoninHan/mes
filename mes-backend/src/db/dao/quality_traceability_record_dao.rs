use crate::db::entity::{self, quality_traceability_records, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug, Default)]
pub struct QualityTraceabilityRecordFilter {
    pub material_id: Option<i64>,
    pub batch_no: Option<String>,
    pub trace_type: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: QualityTraceabilityRecordFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<quality_traceability_records::Model>, u64)> {
    let mut query = entity::QualityTraceabilityRecords::find();

    if let Some(material_id) = filter.material_id {
        query = query.filter(quality_traceability_records::Column::MaterialId.eq(material_id));
    }
    if let Some(ref batch_no) = filter.batch_no {
        query = query.filter(quality_traceability_records::Column::BatchNo.eq(batch_no.clone()));
    }
    if let Some(trace_type) = filter.trace_type {
        query = query.filter(quality_traceability_records::Column::TraceType.eq(trace_type));
    }

    query = query
        .filter(quality_traceability_records::Column::IsDeleted.eq(0))
        .order_by_desc(quality_traceability_records::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<quality_traceability_records::Model>> {
    Ok(entity::QualityTraceabilityRecords::find_by_id(id)
        .filter(quality_traceability_records::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: quality_traceability_records::ActiveModel,
) -> Result<quality_traceability_records::Model> {
    Ok(entity::QualityTraceabilityRecords::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: quality_traceability_records::ActiveModel,
) -> Result<Option<quality_traceability_records::Model>> {
    active.id = Set(id);
    Ok(Some(active.update(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active_model: quality_traceability_records::ActiveModel = entity::QualityTraceabilityRecords::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Record not found"))?
        .into();
    
    active_model.is_deleted = Set(1);
    active_model.updated_time = Set(chrono::Utc::now().into());
    active_model.update(conn).await?;
    Ok(())
}


