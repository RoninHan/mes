use crate::db::entity::{self, equipment_status_log, ConnRef};
use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

#[derive(Debug)]
pub struct StatusLogFilter {
    pub equipment_id: Option<i32>,
    pub status: Option<i32>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: StatusLogFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<equipment_status_log::Model>, u64)> {
    let mut query = equipment_status_log::Entity::find();

    if let Some(eq_id) = filter.equipment_id {
        query = query.filter(equipment_status_log::Column::EquipmentId.eq(eq_id));
    }
    if let Some(s) = filter.status {
        query = query.filter(equipment_status_log::Column::Status.eq(s));
    }
    if let Some(start) = filter.start_time {
        query = query.filter(equipment_status_log::Column::LogTime.gte(start));
    }
    if let Some(end) = filter.end_time {
        query = query.filter(equipment_status_log::Column::LogTime.lte(end));
    }

    query = query.order_by_desc(equipment_status_log::Column::LogTime);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}


