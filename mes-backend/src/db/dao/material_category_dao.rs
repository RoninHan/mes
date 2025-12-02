use crate::db::entity::{self, material_categories, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug)]
pub struct CategoryFilter {
    pub parent_id: Option<i64>,
    pub status: Option<i8>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: CategoryFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<material_categories::Model>, u64)> {
    let mut query = entity::MaterialCategories::find();

    if let Some(parent) = filter.parent_id {
        query = query.filter(material_categories::Column::ParentId.eq(parent));
    }
    if let Some(status) = filter.status {
        query = query.filter(material_categories::Column::Status.eq(status));
    }

    query = query.order_by_asc(material_categories::Column::SortOrder);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<material_categories::Model>> {
    Ok(entity::MaterialCategories::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    mut active: material_categories::ActiveModel,
) -> Result<material_categories::Model> {
    if active.parent_id.is_not_set() {
        active.parent_id = Set(0);
    }
    if active.category_level.is_not_set() {
        active.category_level = Set(1);
    }
    if active.status.is_not_set() {
        active.status = Set(1);
    }
    Ok(entity::MaterialCategories::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: material_categories::ActiveModel,
) -> Result<Option<material_categories::Model>> {
    active.id = Set(id);
    let model = entity::MaterialCategories::update(active)
        .exec_with_returning(conn)
        .await?;
    Ok(Some(model))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = entity::MaterialCategories::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


