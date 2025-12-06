use crate::db::entity::{self, equipment_fault_reports, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug)]
pub struct FaultReportFilter {
    pub equipment_id: Option<i64>,
    pub status: Option<i16>,
    pub fault_level: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: FaultReportFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<equipment_fault_reports::Model>, u64)> {
    let mut query = entity::EquipmentFaultReports::find()
        .filter(equipment_fault_reports::Column::IsDeleted.eq(0));

    if let Some(eid) = filter.equipment_id {
        query = query.filter(equipment_fault_reports::Column::EquipmentId.eq(eid));
    }
    if let Some(status) = filter.status {
        query = query.filter(equipment_fault_reports::Column::Status.eq(status));
    }
    if let Some(level) = filter.fault_level {
        query = query.filter(equipment_fault_reports::Column::FaultLevel.eq(level));
    }

    query = query.order_by_desc(equipment_fault_reports::Column::OccurTime);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<equipment_fault_reports::Model>> {
    Ok(entity::EquipmentFaultReports::find_by_id(id)
        .filter(equipment_fault_reports::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: equipment_fault_reports::ActiveModel,
) -> Result<equipment_fault_reports::Model> {
    Ok(entity::EquipmentFaultReports::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: equipment_fault_reports::ActiveModel,
) -> Result<Option<equipment_fault_reports::Model>> {
    active.id = Set(id);
    Ok(Some(
        entity::EquipmentFaultReports::update(active)
            .exec_with_returning(conn)
            .await?,
    ))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = entity::EquipmentFaultReports::update_many()
        .col_expr(
            equipment_fault_reports::Column::IsDeleted,
            sea_orm::Expr::value(1),
        )
        .filter(equipment_fault_reports::Column::Id.eq(id))
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}



