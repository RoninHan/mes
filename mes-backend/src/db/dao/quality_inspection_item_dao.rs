use crate::db::entity::{self, quality_inspection_items, ConnRef};
use anyhow::Result;
use sea_orm::{QueryOrder,
    ColumnTrait, EntityTrait, QueryFilter, Set};

pub async fn list_by_report_id(
    conn: ConnRef<'_>,
    report_id: i64,
) -> Result<Vec<quality_inspection_items::Model>> {
    Ok(quality_inspection_items::Entity::find()
        .filter(quality_inspection_items::Column::ReportId.eq(report_id))
        .filter(quality_inspection_items::Column::IsDeleted.eq(0))
        .order_by_asc(quality_inspection_items::Column::SequenceNo)
        .all(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: quality_inspection_items::ActiveModel,
) -> Result<quality_inspection_items::Model> {
    let res = quality_inspection_items::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(quality_inspection_items::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn create_batch(
    conn: ConnRef<'_>,
    items: Vec<quality_inspection_items::ActiveModel>,
) -> Result<Vec<quality_inspection_items::Model>> {
    let mut results = Vec::new();
    for item in items {
        let res = quality_inspection_items::Entity::insert(item)
            .exec(conn)
            .await?;
        let model = quality_inspection_items::Entity::find_by_id(res.last_insert_id)
            .one(conn)
            .await?
            .expect("just inserted");
        results.push(model);
    }
    Ok(results)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: quality_inspection_items::ActiveModel,
) -> Result<Option<quality_inspection_items::Model>> {
    active.id = Set(id);
    Ok(Some(quality_inspection_items::Entity::update(active).exec(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active_model: quality_inspection_items::ActiveModel = quality_inspection_items::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Item not found"))?
        .into();
    
    active_model.is_deleted = Set(1);
    active_model.updated_time = Set(chrono::Utc::now().into());
    quality_inspection_items::Entity::update(active_model).exec(conn).await?;
    Ok(())
}

pub async fn delete_by_report_id(conn: ConnRef<'_>, report_id: i64) -> Result<()> {
    let items = quality_inspection_items::Entity::find()
        .filter(quality_inspection_items::Column::ReportId.eq(report_id))
        .filter(quality_inspection_items::Column::IsDeleted.eq(0))
        .all(conn)
        .await?;
    
    for item in items {
        let mut active_model: quality_inspection_items::ActiveModel = item.into();
        active_model.is_deleted = Set(1);
        active_model.updated_time = Set(chrono::Utc::now().into());
        quality_inspection_items::Entity::update(active_model).exec(conn).await?;
    }
    Ok(())
}


