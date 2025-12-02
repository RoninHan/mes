use crate::db::entity::{self, work_orders, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug)]
pub struct WorkOrderFilter {
    pub work_order_status: Option<i8>,
    pub equipment_id: Option<i64>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: WorkOrderFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<work_orders::Model>, u64)> {
    let mut query = entity::WorkOrders::find();

    if let Some(status) = filter.work_order_status {
        query = query.filter(work_orders::Column::WorkOrderStatus.eq(status));
    }
    if let Some(equipment) = filter.equipment_id {
        query = query.filter(work_orders::Column::EquipmentId.eq(equipment));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        query = query.filter(work_orders::Column::WorkOrderNo.ilike(like));
    }

    query = query.order_by_desc(work_orders::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<work_orders::Model>> {
    Ok(entity::WorkOrders::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: work_orders::ActiveModel,
) -> Result<work_orders::Model> {
    Ok(entity::WorkOrders::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: work_orders::ActiveModel,
) -> Result<Option<work_orders::Model>> {
    active.id = Set(id);
    let updated = entity::WorkOrders::update(active)
        .exec_with_returning(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = entity::WorkOrders::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


