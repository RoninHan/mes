use crate::db::entity::{self, equipment_calibration_records, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug, Default)]
pub struct EquipmentCalibrationRecordFilter {
    pub equipment_id: Option<i64>,
    pub calibration_result: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: EquipmentCalibrationRecordFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<equipment_calibration_records::Model>, u64)> {
    let mut query = entity::EquipmentCalibrationRecords::find();

    if let Some(equipment_id) = filter.equipment_id {
        query = query.filter(equipment_calibration_records::Column::EquipmentId.eq(equipment_id));
    }
    if let Some(calibration_result) = filter.calibration_result {
        query = query.filter(equipment_calibration_records::Column::CalibrationResult.eq(calibration_result));
    }

    query = query
        .filter(equipment_calibration_records::Column::IsDeleted.eq(0))
        .order_by_desc(equipment_calibration_records::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<equipment_calibration_records::Model>> {
    Ok(entity::EquipmentCalibrationRecords::find_by_id(id)
        .filter(equipment_calibration_records::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: equipment_calibration_records::ActiveModel,
) -> Result<equipment_calibration_records::Model> {
    Ok(entity::EquipmentCalibrationRecords::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: equipment_calibration_records::ActiveModel,
) -> Result<Option<equipment_calibration_records::Model>> {
    active.id = Set(id);
    Ok(Some(active.update(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active_model: equipment_calibration_records::ActiveModel = entity::EquipmentCalibrationRecords::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Record not found"))?
        .into();
    
    active_model.is_deleted = Set(1);
    active_model.updated_time = Set(chrono::Utc::now().into());
    active_model.update(conn).await?;
    Ok(())
}


