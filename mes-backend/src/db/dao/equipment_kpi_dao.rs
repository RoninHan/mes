use crate::db::entity::{self, equipment_kpi, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

#[derive(Debug)]
pub struct EquipmentKpiFilter {
    pub equipment_id: Option<i64>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: EquipmentKpiFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<equipment_kpi::Model>, u64)> {
    let mut query = entity::EquipmentKpi::find()
        .filter(equipment_kpi::Column::IsDeleted.eq(0));

    if let Some(eid) = filter.equipment_id {
        query = query.filter(equipment_kpi::Column::EquipmentId.eq(eid));
    }

    query = query
        .order_by_desc(equipment_kpi::Column::StatDate)
        .order_by_desc(equipment_kpi::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}



