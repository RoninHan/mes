use crate::db::entity::{self, operation_logs, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

#[derive(Debug)]
pub struct OperationLogFilter {
    pub user_id: Option<i64>,
    pub module: Option<String>,
    pub action: Option<String>,
    pub success: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: OperationLogFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<operation_logs::Model>, u64)> {
    let mut query = entity::OperationLogs::find();

    if let Some(user_id) = filter.user_id {
        query = query.filter(operation_logs::Column::UserId.eq(user_id));
    }
    if let Some(module) = filter.module {
        query = query.filter(operation_logs::Column::Module.eq(module));
    }
    if let Some(action) = filter.action {
        query = query.filter(operation_logs::Column::Action.eq(action));
    }
    if let Some(success) = filter.success {
        query = query.filter(operation_logs::Column::Success.eq(success));
    }

    query = query.order_by_desc(operation_logs::Column::RequestTime);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn create(
    conn: ConnRef<'_>,
    active: operation_logs::ActiveModel,
) -> Result<operation_logs::Model> {
    Ok(entity::OperationLogs::insert(active)
        .exec_with_returning(conn)
        .await?)
}



