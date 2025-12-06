use crate::db::entity::{self, workshops, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug, Default)]
pub struct WorkshopFilter {
    pub status: Option<i16>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: WorkshopFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<workshops::Model>, u64)> {
    let mut query = entity::Workshops::find().filter(workshops::Column::IsDeleted.eq(0));

    if let Some(status) = filter.status {
        query = query.filter(workshops::Column::Status.eq(status));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        query = query.filter(
            workshops::Column::WorkshopName
                .ilike(like.clone())
                .or(workshops::Column::WorkshopCode.ilike(like)),
        );
    }

    query = query.order_by_desc(workshops::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<workshops::Model>> {
    Ok(entity::Workshops::find_by_id(id)
        .filter(workshops::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: workshops::ActiveModel,
) -> Result<workshops::Model> {
    Ok(entity::Workshops::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: workshops::ActiveModel,
) -> Result<Option<workshops::Model>> {
    active.id = Set(id);
    Ok(Some(active.update(conn).await?))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<()> {
    let mut active: workshops::ActiveModel = entity::Workshops::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Workshop not found"))?
        .into();

    active.is_deleted = Set(1);
    active.updated_time = Set(chrono::Utc::now().into());
    active.update(conn).await?;
    Ok(())
}



