use crate::db::entity::{self, quality_inspection_tasks, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, ActiveModelTrait};

#[derive(Debug, Default)]
pub struct QualityInspectionTaskFilter {
    pub inspection_type: Option<i16>,
    pub source_type: Option<i16>,
    pub source_order_no: Option<String>,
    pub material_id: Option<i64>,
    pub batch_no: Option<String>,
    pub task_status: Option<i16>,
    pub inspector_id: Option<i64>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: QualityInspectionTaskFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<quality_inspection_tasks::Model>, u64)> {
    let mut query = quality_inspection_tasks::Entity::find();

    if let Some(inspection_type) = filter.inspection_type {
        query = query.filter(quality_inspection_tasks::Column::InspectionType.eq(inspection_type));
    }
    if let Some(source_type) = filter.source_type {
        query = query.filter(quality_inspection_tasks::Column::SourceType.eq(source_type));
    }
    if let Some(ref source_order_no) = filter.source_order_no {
        query = query.filter(quality_inspection_tasks::Column::SourceOrderNo.eq(source_order_no.clone()));
    }
    if let Some(material_id) = filter.material_id {
        query = query.filter(quality_inspection_tasks::Column::MaterialId.eq(material_id));
    }
    if let Some(ref batch_no) = filter.batch_no {
        query = query.filter(quality_inspection_tasks::Column::BatchNo.eq(batch_no.clone()));
    }
    if let Some(task_status) = filter.task_status {
        query = query.filter(quality_inspection_tasks::Column::TaskStatus.eq(task_status));
    }
    if let Some(inspector_id) = filter.inspector_id {
        query = query.filter(quality_inspection_tasks::Column::InspectorId.eq(inspector_id));
    }

    query = query
        .filter(quality_inspection_tasks::Column::IsDeleted.eq(0))
        .order_by_desc(quality_inspection_tasks::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<quality_inspection_tasks::Model>> {
    Ok(quality_inspection_tasks::Entity::find_by_id(id)
        .filter(quality_inspection_tasks::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    model: quality_inspection_tasks::ActiveModel,
) -> Result<quality_inspection_tasks::Model> {
    Ok(model.insert(conn).await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    model: quality_inspection_tasks::ActiveModel,
) -> Result<quality_inspection_tasks::Model> {
    let mut active_model: quality_inspection_tasks::ActiveModel = quality_inspection_tasks::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Task not found"))?
        .into();
    
    // Update fields
    if let Set(task_no) = model.task_no {
        active_model.task_no = Set(task_no);
    }
    if let Set(inspection_type) = model.inspection_type {
        active_model.inspection_type = Set(inspection_type);
    }
    if let Set(source_type) = model.source_type {
        active_model.source_type = Set(source_type);
    }
    if let Set(source_order_no) = model.source_order_no {
        active_model.source_order_no = Set(source_order_no);
    }
    if let Set(material_id) = model.material_id {
        active_model.material_id = Set(material_id);
    }
    if let Set(batch_no) = model.batch_no {
        active_model.batch_no = Set(batch_no);
    }
    if let Set(task_status) = model.task_status {
        active_model.task_status = Set(task_status);
    }
    if let Set(inspection_result) = model.inspection_result {
        active_model.inspection_result = Set(inspection_result);
    }
    if let Set(actual_start_time) = model.actual_start_time {
        active_model.actual_start_time = Set(actual_start_time);
    }
    if let Set(actual_end_time) = model.actual_end_time {
        active_model.actual_end_time = Set(actual_end_time);
    }
    if let Set(qualified_quantity) = model.qualified_quantity {
        active_model.qualified_quantity = Set(qualified_quantity);
    }
    if let Set(unqualified_quantity) = model.unqualified_quantity {
        active_model.unqualified_quantity = Set(unqualified_quantity);
    }
    if let Set(remark) = model.remark {
        active_model.remark = Set(remark);
    }
    if let Set(updated_by) = model.updated_by {
        active_model.updated_by = Set(updated_by);
    }
    active_model.updated_time = Set(chrono::Utc::now().into());

    Ok(active_model.update(conn).await?)
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active_model: quality_inspection_tasks::ActiveModel = quality_inspection_tasks::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Task not found"))?
        .into();
    
    active_model.is_deleted = Set(1);
    active_model.updated_time = Set(chrono::Utc::now().into());
    active_model.update(conn).await?;
    Ok(())
}


