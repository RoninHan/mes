use crate::db::entity::{self, equipment_repair_orders, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};
use sea_orm_migration::sea_query::Expr;

#[derive(Debug)]
pub struct RepairOrderFilter {
    pub equipment_id: Option<i64>,
    pub status: Option<i32>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: RepairOrderFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<equipment_repair_orders::Model>, u64)> {
    let mut query = equipment_repair_orders::Entity::find()
        .filter(equipment_repair_orders::Column::IsDeleted.eq(0));

    if let Some(eid) = filter.equipment_id {
        query = query.filter(equipment_repair_orders::Column::EquipmentId.eq(eid));
    }
    if let Some(status) = filter.status {
        query = query.filter(equipment_repair_orders::Column::Status.eq(status));
    }

    query = query.order_by_desc(equipment_repair_orders::Column::StartTime);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<equipment_repair_orders::Model>> {
    Ok(equipment_repair_orders::Entity::find_by_id(id)
        .filter(equipment_repair_orders::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: equipment_repair_orders::ActiveModel,
) -> Result<equipment_repair_orders::Model> {
    let res = equipment_repair_orders::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(equipment_repair_orders::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: equipment_repair_orders::ActiveModel,
) -> Result<Option<equipment_repair_orders::Model>> {
    active.id = Set(id);
    Ok(Some(
        equipment_repair_orders::Entity::update(active)
            .exec(conn)
            .await?,
    ))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = equipment_repair_orders::Entity::update_many()
        .col_expr(
            equipment_repair_orders::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(equipment_repair_orders::Column::Id.eq(id))
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}



