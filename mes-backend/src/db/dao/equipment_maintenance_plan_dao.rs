use crate::db::entity::{self, equipment_maintenance_plans, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};
use sea_orm_migration::sea_query::Expr;

#[derive(Debug)]
pub struct MaintenancePlanFilter {
    pub equipment_id: Option<i64>,
    pub status: Option<i32>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: MaintenancePlanFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<equipment_maintenance_plans::Model>, u64)> {
    let mut query = equipment_maintenance_plans::Entity::find()
        .filter(equipment_maintenance_plans::Column::IsDeleted.eq(0));

    if let Some(eid) = filter.equipment_id {
        query = query.filter(equipment_maintenance_plans::Column::EquipmentId.eq(eid));
    }
    if let Some(status) = filter.status {
        query = query.filter(equipment_maintenance_plans::Column::Status.eq(status));
    }

    query = query.order_by_desc(equipment_maintenance_plans::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<equipment_maintenance_plans::Model>> {
    Ok(equipment_maintenance_plans::Entity::find_by_id(id)
        .filter(equipment_maintenance_plans::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: equipment_maintenance_plans::ActiveModel,
) -> Result<equipment_maintenance_plans::Model> {
    let res = equipment_maintenance_plans::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(equipment_maintenance_plans::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: equipment_maintenance_plans::ActiveModel,
) -> Result<Option<equipment_maintenance_plans::Model>> {
    active.id = Set(id);
    Ok(Some(
        equipment_maintenance_plans::Entity::update(active)
            .exec(conn)
            .await?,
    ))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = equipment_maintenance_plans::Entity::update_many()
        .col_expr(
            equipment_maintenance_plans::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(equipment_maintenance_plans::Column::Id.eq(id))
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}



