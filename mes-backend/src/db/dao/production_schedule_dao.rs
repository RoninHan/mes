use crate::db::entity::{self, production_schedules, ConnRef};
use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug)]
pub struct ScheduleFilter {
    pub workshop_id: Option<i64>,
    pub equipment_id: Option<i64>,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

pub async fn list_by_window(
    conn: ConnRef<'_>,
    filter: ScheduleFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<production_schedules::Model>, u64)> {
    let mut q = production_schedules::Entity::find();

    if let Some(ws) = filter.workshop_id {
        q = q.filter(production_schedules::Column::WorkshopId.eq(ws));
    }
    if let Some(eq) = filter.equipment_id {
        q = q.filter(production_schedules::Column::EquipmentId.eq(eq));
    }

    // overlap [from, to]
    q = q.filter(
        production_schedules::Column::StartTime
            .lte(filter.to)
            .and(production_schedules::Column::EndTime.gte(filter.from)),
    );

    q = q.order_by_asc(production_schedules::Column::StartTime);

    let paginator = q.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i32,
) -> Result<Option<production_schedules::Model>> {
    Ok(production_schedules::Entity::find_by_id(id)
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    mut active: production_schedules::ActiveModel,
) -> Result<production_schedules::Model> {
    if active.status.is_not_set() {
        active.status = Set(1);
    }
    if active.priority.is_not_set() {
        active.priority = Set(3);
    }
    Ok(production_schedules::Entity::insert(active)
        .exec(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i32,
    mut active: production_schedules::ActiveModel,
) -> Result<Option<production_schedules::Model>> {
    active.id = Set(id);
    let updated = production_schedules::Entity::update(active)
        .exec(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i32) -> Result<u64> {
    let res = production_schedules::Entity::delete_by_id(id)
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}


