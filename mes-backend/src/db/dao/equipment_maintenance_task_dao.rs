use crate::db::entity::{self, equipment_maintenance_tasks, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};
use sea_orm_migration::sea_query::Expr;

#[derive(Debug)]
pub struct MaintenanceTaskFilter {
    pub equipment_id: Option<i64>,
    pub status: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: MaintenanceTaskFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<equipment_maintenance_tasks::Model>, u64)> {
    let mut query = equipment_maintenance_tasks::Entity::find()
        .filter(equipment_maintenance_tasks::Column::IsDeleted.eq(0));

    if let Some(eid) = filter.equipment_id {
        query = query.filter(equipment_maintenance_tasks::Column::EquipmentId.eq(eid));
    }
    if let Some(status) = filter.status {
        query = query.filter(equipment_maintenance_tasks::Column::Status.eq(status));
    }

    query = query.order_by_desc(equipment_maintenance_tasks::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<equipment_maintenance_tasks::Model>> {
    Ok(equipment_maintenance_tasks::Entity::find_by_id(id)
        .filter(equipment_maintenance_tasks::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: equipment_maintenance_tasks::ActiveModel,
) -> Result<equipment_maintenance_tasks::Model> {
    Ok(equipment_maintenance_tasks::Entity::insert(active)
        .exec(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: equipment_maintenance_tasks::ActiveModel,
) -> Result<Option<equipment_maintenance_tasks::Model>> {
    active.id = Set(id);
    Ok(Some(
        equipment_maintenance_tasks::Entity::update(active)
            .exec(conn)
            .await?,
    ))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = equipment_maintenance_tasks::Entity::update_many()
        .col_expr(
            equipment_maintenance_tasks::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(equipment_maintenance_tasks::Column::Id.eq(id))
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}



