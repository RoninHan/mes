use crate::db::entity::{self, users, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug)]
pub struct UserFilter {
    pub dept_id: Option<i64>,
    pub status: Option<i32>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: UserFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<users::Model>, u64)> {
    let mut q = users::Entity::find();

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
                .like(like.clone())
                .or(users::Column::RealName.like(like)),
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
    Ok(users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(conn)
        .await?)
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<users::Model>> {
    Ok(users::Entity::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: users::ActiveModel,
) -> Result<users::Model> {
    let res = users::Entity::insert(active)
        .exec(conn)
        .await?;
    Ok(users::Entity::find_by_id(res.last_insert_id)
        .one(conn)
        .await?
        .expect("just inserted"))
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: users::ActiveModel,
) -> Result<Option<users::Model>> {
    active.id = Set(id);
    let updated = users::Entity::update(active)
        .exec(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = users::Entity::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


