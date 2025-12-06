use crate::db::entity::{self, login_logs, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

#[derive(Debug)]
pub struct LoginLogFilter {
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub result: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: LoginLogFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<login_logs::Model>, u64)> {
    let mut query = entity::LoginLogs::find();

    if let Some(user_id) = filter.user_id {
        query = query.filter(login_logs::Column::UserId.eq(user_id));
    }
    if let Some(username) = filter.username {
        query = query.filter(login_logs::Column::Username.eq(username));
    }
    if let Some(result) = filter.result {
        query = query.filter(login_logs::Column::Result.eq(result));
    }

    query = query.order_by_desc(login_logs::Column::LoginTime);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn create(
    conn: ConnRef<'_>,
    active: login_logs::ActiveModel,
) -> Result<login_logs::Model> {
    Ok(entity::LoginLogs::insert(active)
        .exec_with_returning(conn)
        .await?)
}



