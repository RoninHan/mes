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
    let mut q = entity::Roles::find();
    if let Some(status) = filter.status {
        q = q.filter(roles::Column::Status.eq(status));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        q = q.filter(
            roles::Column::RoleName
                .ilike(like.clone())
                .or(roles::Column::RoleCode.ilike(like)),
        );
    }
    q = q.order_by_desc(roles::Column::Id);
    let paginator = q.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<roles::Model>> {
    Ok(entity::Roles::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: roles::ActiveModel,
) -> Result<roles::Model> {
    Ok(entity::Roles::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: roles::ActiveModel,
) -> Result<Option<roles::Model>> {
    active.id = Set(id);
    let updated = entity::Roles::update(active)
        .exec_with_returning(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = entity::Roles::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


