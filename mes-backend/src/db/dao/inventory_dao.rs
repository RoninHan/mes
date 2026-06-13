use crate::db::entity::{self, inventory, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

#[derive(Debug)]
pub struct InventoryFilter {
    pub material_id: Option<i64>,
    pub warehouse_id: Option<i64>,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: InventoryFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<inventory::Model>, u64)> {
    let mut query = inventory::Entity::find();

    if let Some(material_id) = filter.material_id {
        query = query.filter(inventory::Column::MaterialId.eq(material_id));
    }
    if let Some(warehouse_id) = filter.warehouse_id {
        query = query.filter(inventory::Column::WarehouseId.eq(warehouse_id));
    }
    if let Some(location_id) = filter.location_id {
        query = query.filter(inventory::Column::LocationId.eq(location_id));
    }
    if let Some(batch) = filter.batch_no {
        query = query.filter(inventory::Column::BatchNo.eq(batch));
    }

    query = query.order_by_desc(inventory::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<inventory::Model>> {
    Ok(inventory::Entity::find_by_id(id).one(conn).await?)
}


