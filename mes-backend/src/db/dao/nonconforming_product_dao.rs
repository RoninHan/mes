use crate::db::entity::{self, nonconforming_products, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug, Default)]
pub struct NonconformingProductFilter {
    pub ncr_status: Option<i32>,
    pub material_id: Option<i64>,
    pub batch_no: Option<String>,
    pub defect_level: Option<i32>,
    pub source_type: Option<i32>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: NonconformingProductFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<nonconforming_products::Model>, u64)> {
    let mut query = nonconforming_products::Entity::find();

    if let Some(ncr_status) = filter.ncr_status {
        query = query.filter(nonconforming_products::Column::NcrStatus.eq(ncr_status));
    }
    if let Some(material_id) = filter.material_id {
        query = query.filter(nonconforming_products::Column::MaterialId.eq(material_id));
    }
    if let Some(ref batch_no) = filter.batch_no {
        query = query.filter(nonconforming_products::Column::BatchNo.eq(batch_no.clone()));
    }
    if let Some(defect_level) = filter.defect_level {
        query = query.filter(nonconforming_products::Column::DefectLevel.eq(defect_level));
    }
    if let Some(source_type) = filter.source_type {
        query = query.filter(nonconforming_products::Column::SourceType.eq(source_type));
    }

    query = query
        .filter(nonconforming_products::Column::IsDeleted.eq(0))
        .order_by_desc(nonconforming_products::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<nonconforming_products::Model>> {
    Ok(nonconforming_products::Entity::find_by_id(id)
        .filter(nonconforming_products::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: nonconforming_products::ActiveModel,
) -> Result<nonconforming_products::Model> {
    let res = nonconforming_products::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(nonconforming_products::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: nonconforming_products::ActiveModel,
) -> Result<Option<nonconforming_products::Model>> {
    active.id = Set(id);
    Ok(Some(nonconforming_products::Entity::update(active).exec(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active_model: nonconforming_products::ActiveModel = nonconforming_products::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("NCR not found"))?
        .into();
    
    active_model.is_deleted = Set(1);
    active_model.updated_time = Set(chrono::Utc::now().into());
    nonconforming_products::Entity::update(active_model).exec(conn).await?;
    Ok(())
}


