use crate::db::entity::{self, warehouses, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug, Default)]
pub struct WarehouseFilter {
    pub warehouse_type: Option<i32>,
    pub status: Option<i32>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: WarehouseFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<warehouses::Model>, u64)> {
    let mut query = warehouses::Entity::find().filter(warehouses::Column::IsDeleted.eq(0));

    if let Some(warehouse_type) = filter.warehouse_type {
        query = query.filter(warehouses::Column::WarehouseType.eq(warehouse_type));
    }
    if let Some(status) = filter.status {
        query = query.filter(warehouses::Column::Status.eq(status));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        query = query.filter(
            warehouses::Column::WarehouseName
                .like(like.clone())
                .or(warehouses::Column::WarehouseCode.like(like)),
        );
    }

    query = query.order_by_desc(warehouses::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<warehouses::Model>> {
    Ok(warehouses::Entity::find_by_id(id)
        .filter(warehouses::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: warehouses::ActiveModel,
) -> Result<warehouses::Model> {
    let res = warehouses::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(warehouses::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: warehouses::ActiveModel,
) -> Result<Option<warehouses::Model>> {
    active.id = Set(id);
    Ok(Some(warehouses::Entity::update(active).exec(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active: warehouses::ActiveModel = warehouses::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Warehouse not found"))?
        .into();

    active.is_deleted = Set(1);
    active.updated_time = Set(chrono::Utc::now().into());
    warehouses::Entity::update(active).exec(conn).await?;
    Ok(())
}



