use crate::db::entity::{self, equipment_inspections, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};
use sea_orm_migration::sea_query::Expr;

#[derive(Debug)]
pub struct EquipmentInspectionFilter {
    pub equipment_id: Option<i64>,
    pub inspection_type: Option<i32>,
    pub result: Option<i32>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: EquipmentInspectionFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<equipment_inspections::Model>, u64)> {
    let mut query = equipment_inspections::Entity::find()
        .filter(equipment_inspections::Column::IsDeleted.eq(0));

    if let Some(eid) = filter.equipment_id {
        query = query.filter(equipment_inspections::Column::EquipmentId.eq(eid));
    }
    if let Some(t) = filter.inspection_type {
        query = query.filter(equipment_inspections::Column::InspectionType.eq(t));
    }
    if let Some(r) = filter.result {
        query = query.filter(equipment_inspections::Column::Result.eq(r));
    }

    query = query.order_by_desc(equipment_inspections::Column::InspectionTime);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<equipment_inspections::Model>> {
    Ok(equipment_inspections::Entity::find_by_id(id)
        .filter(equipment_inspections::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: equipment_inspections::ActiveModel,
) -> Result<equipment_inspections::Model> {
    let res = equipment_inspections::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(equipment_inspections::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: equipment_inspections::ActiveModel,
) -> Result<Option<equipment_inspections::Model>> {
    active.id = Set(id);
    Ok(Some(
        equipment_inspections::Entity::update(active)
            .exec(conn)
            .await?,
    ))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = equipment_inspections::Entity::update_many()
        .col_expr(
            equipment_inspections::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(equipment_inspections::Column::Id.eq(id))
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}



