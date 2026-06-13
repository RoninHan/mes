use crate::db::entity::{self, roles, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug)]
pub struct RoleFilter {
    pub status: Option<i8>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: RoleFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<roles::Model>, u64)> {
    let mut q = roles::Entity::find();
    if let Some(status) = filter.status {
        q = q.filter(roles::Column::Status.eq(status));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        q = q.filter(
            roles::Column::RoleName
                .like(like.clone())
                .or(roles::Column::RoleCode.like(like)),
        );
    }
    q = q.order_by_desc(roles::Column::Id);
    let paginator = q.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<roles::Model>> {
    Ok(roles::Entity::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: roles::ActiveModel,
) -> Result<roles::Model> {
    Ok(roles::Entity::insert(active)
        .exec(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: roles::ActiveModel,
) -> Result<Option<roles::Model>> {
    active.id = Set(id);
    let updated = roles::Entity::update(active)
        .exec(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = roles::Entity::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


