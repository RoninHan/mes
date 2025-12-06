use crate::db::entity::{self, batch_traceability, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

#[derive(Debug)]
pub struct BatchTraceFilter {
    pub material_id: Option<i64>,
    pub batch_no: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: BatchTraceFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<batch_traceability::Model>, u64)> {
    let mut query = entity::BatchTraceability::find()
        .filter(batch_traceability::Column::IsDeleted.eq(0));

    if let Some(m) = filter.material_id {
        query = query.filter(batch_traceability::Column::MaterialId.eq(m));
    }
    if let Some(b) = filter.batch_no {
        query = query.filter(batch_traceability::Column::BatchNo.eq(b));
    }

    query = query.order_by_desc(batch_traceability::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}


