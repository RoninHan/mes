use crate::db::entity::{self, users, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug)]
pub struct UserFilter {
    pub dept_id: Option<i64>,
    pub status: Option<i8>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: UserFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<users::Model>, u64)> {
    let mut q = entity::Users::find();

    if let Some(dept_id) = filter.dept_id {
        q = q.filter(users::Column::DeptId.eq(dept_id));
    }
    if let Some(status) = filter.status {
        q = q.filter(users::Column::Status.eq(status));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        q = q.filter(
            users::Column::Username
                .ilike(like.clone())
                .or(users::Column::RealName.ilike(like)),
        );
    }

    q = q.order_by_desc(users::Column::Id);

    let paginator = q.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn find_by_username(
    conn: ConnRef<'_>,
    username: &str,
) -> Result<Option<users::Model>> {
    Ok(entity::Users::find()
        .filter(users::Column::Username.eq(username))
        .one(conn)
        .await?)
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<users::Model>> {
    Ok(entity::Users::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: users::ActiveModel,
) -> Result<users::Model> {
    Ok(entity::Users::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: users::ActiveModel,
) -> Result<Option<users::Model>> {
    active.id = Set(id);
    let updated = entity::Users::update(active)
        .exec_with_returning(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = entity::Users::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


