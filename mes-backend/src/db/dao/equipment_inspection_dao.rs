use crate::db::entity::{self, equipment_inspections, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug)]
pub struct EquipmentInspectionFilter {
    pub equipment_id: Option<i64>,
    pub inspection_type: Option<i16>,
    pub result: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: EquipmentInspectionFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<equipment_inspections::Model>, u64)> {
    let mut query = entity::EquipmentInspections::find()
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
    Ok(entity::EquipmentInspections::find_by_id(id)
        .filter(equipment_inspections::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: equipment_inspections::ActiveModel,
) -> Result<equipment_inspections::Model> {
    Ok(entity::EquipmentInspections::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: equipment_inspections::ActiveModel,
) -> Result<Option<equipment_inspections::Model>> {
    active.id = Set(id);
    Ok(Some(
        entity::EquipmentInspections::update(active)
            .exec_with_returning(conn)
            .await?,
    ))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = entity::EquipmentInspections::update_many()
        .col_expr(
            equipment_inspections::Column::IsDeleted,
            sea_orm::Expr::value(1),
        )
        .filter(equipment_inspections::Column::Id.eq(id))
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}



