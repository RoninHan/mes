use crate::db::entity::{self, measuring_equipment, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug, Default)]
pub struct MeasuringEquipmentFilter {
    pub equipment_type: Option<i32>,
    pub equipment_status: Option<i32>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: MeasuringEquipmentFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<measuring_equipment::Model>, u64)> {
    let mut query = measuring_equipment::Entity::find();

    if let Some(equipment_type) = filter.equipment_type {
        query = query.filter(measuring_equipment::Column::EquipmentType.eq(equipment_type));
    }
    if let Some(equipment_status) = filter.equipment_status {
        query = query.filter(measuring_equipment::Column::EquipmentStatus.eq(equipment_status));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        query = query.filter(
            measuring_equipment::Column::EquipmentCode
                .like(like.clone())
                .or(measuring_equipment::Column::EquipmentName.like(like)),
        );
    }

    query = query
        .filter(measuring_equipment::Column::IsDeleted.eq(0))
        .order_by_desc(measuring_equipment::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<measuring_equipment::Model>> {
    Ok(measuring_equipment::Entity::find_by_id(id)
        .filter(measuring_equipment::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: measuring_equipment::ActiveModel,
) -> Result<measuring_equipment::Model> {
    let res = measuring_equipment::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(measuring_equipment::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: measuring_equipment::ActiveModel,
) -> Result<Option<measuring_equipment::Model>> {
    active.id = Set(id);
    Ok(Some(measuring_equipment::Entity::update(active).exec(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active_model: measuring_equipment::ActiveModel = measuring_equipment::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Equipment not found"))?
        .into();
    
    active_model.is_deleted = Set(1);
    active_model.updated_time = Set(chrono::Utc::now().into());
    measuring_equipment::Entity::update(active_model).exec(conn).await?;
    Ok(())
}


