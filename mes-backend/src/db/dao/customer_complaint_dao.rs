use crate::db::entity::{self, customer_complaints, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug, Default)]
pub struct CustomerComplaintFilter {
    pub customer_id: Option<i64>,
    pub complaint_status: Option<i16>,
    pub complaint_type: Option<i16>,
    pub material_id: Option<i64>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: CustomerComplaintFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<customer_complaints::Model>, u64)> {
    let mut query = entity::CustomerComplaints::find();

    if let Some(customer_id) = filter.customer_id {
        query = query.filter(customer_complaints::Column::CustomerId.eq(customer_id));
    }
    if let Some(complaint_status) = filter.complaint_status {
        query = query.filter(customer_complaints::Column::ComplaintStatus.eq(complaint_status));
    }
    if let Some(complaint_type) = filter.complaint_type {
        query = query.filter(customer_complaints::Column::ComplaintType.eq(complaint_type));
    }
    if let Some(material_id) = filter.material_id {
        query = query.filter(customer_complaints::Column::MaterialId.eq(material_id));
    }

    query = query
        .filter(customer_complaints::Column::IsDeleted.eq(0))
        .order_by_desc(customer_complaints::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<customer_complaints::Model>> {
    Ok(entity::CustomerComplaints::find_by_id(id)
        .filter(customer_complaints::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: customer_complaints::ActiveModel,
) -> Result<customer_complaints::Model> {
    Ok(entity::CustomerComplaints::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: customer_complaints::ActiveModel,
) -> Result<Option<customer_complaints::Model>> {
    active.id = Set(id);
    Ok(Some(active.update(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active_model: customer_complaints::ActiveModel = entity::CustomerComplaints::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Complaint not found"))?
        .into();
    
    active_model.is_deleted = Set(1);
    active_model.updated_time = Set(chrono::Utc::now().into());
    active_model.update(conn).await?;
    Ok(())
}


