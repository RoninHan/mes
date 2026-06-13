use crate::db::entity::{self, materials, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug)]
pub struct MaterialsFilter {
    pub category_id: Option<i64>,
    pub material_type: Option<i8>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: MaterialsFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<materials::Model>, u64)> {
    let mut query = materials::Entity::find();

    if let Some(category_id) = filter.category_id {
        query = query.filter(materials::Column::CategoryId.eq(category_id));
    }
    if let Some(material_type) = filter.material_type {
        query = query.filter(materials::Column::MaterialType.eq(material_type));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        query = query.filter(
            materials::Column::MaterialName
                .like(like.clone())
                .or(materials::Column::MaterialCode.like(like)),
        );
    }

    query = query.order_by_desc(materials::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<materials::Model>> {
    Ok(materials::Entity::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: materials::ActiveModel,
) -> Result<materials::Model> {
    Ok(materials::Entity::insert(active)
        .exec(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: materials::ActiveModel,
) -> Result<Option<materials::Model>> {
    active.id = Set(id);
    let updated = materials::Entity::update(active)
        .exec(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = materials::Entity::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


