use crate::db::entity::{self, quality_inspection_reports, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, ActiveModelTrait};

#[derive(Debug, Default)]
pub struct QualityInspectionReportFilter {
    pub task_id: Option<i64>,
    pub inspection_type: Option<i16>,
    pub material_id: Option<i64>,
    pub batch_no: Option<String>,
    pub report_status: Option<i16>,
    pub inspection_result: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: QualityInspectionReportFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<quality_inspection_reports::Model>, u64)> {
    let mut query = quality_inspection_reports::Entity::find();

    if let Some(task_id) = filter.task_id {
        query = query.filter(quality_inspection_reports::Column::TaskId.eq(task_id));
    }
    if let Some(inspection_type) = filter.inspection_type {
        query = query.filter(quality_inspection_reports::Column::InspectionType.eq(inspection_type));
    }
    if let Some(material_id) = filter.material_id {
        query = query.filter(quality_inspection_reports::Column::MaterialId.eq(material_id));
    }
    if let Some(ref batch_no) = filter.batch_no {
        query = query.filter(quality_inspection_reports::Column::BatchNo.eq(batch_no.clone()));
    }
    if let Some(report_status) = filter.report_status {
        query = query.filter(quality_inspection_reports::Column::ReportStatus.eq(report_status));
    }
    if let Some(inspection_result) = filter.inspection_result {
        query = query.filter(quality_inspection_reports::Column::InspectionResult.eq(inspection_result));
    }

    query = query
        .filter(quality_inspection_reports::Column::IsDeleted.eq(0))
        .order_by_desc(quality_inspection_reports::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<quality_inspection_reports::Model>> {
    Ok(quality_inspection_reports::Entity::find_by_id(id)
        .filter(quality_inspection_reports::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    model: quality_inspection_reports::ActiveModel,
) -> Result<quality_inspection_reports::Model> {
    Ok(model.insert(conn).await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: quality_inspection_reports::ActiveModel,
) -> Result<Option<quality_inspection_reports::Model>> {
    active.id = Set(id);
    Ok(Some(active.update(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active_model: quality_inspection_reports::ActiveModel = quality_inspection_reports::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Report not found"))?
        .into();
    
    active_model.is_deleted = Set(1);
    active_model.updated_time = Set(chrono::Utc::now().into());
    active_model.update(conn).await?;
    Ok(())
}


