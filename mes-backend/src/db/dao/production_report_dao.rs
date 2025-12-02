use crate::db::entity::{self, production_reports, ConnRef};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug)]
pub struct ReportFilter {
    pub report_type: Option<i8>,
    pub operator_id: Option<i64>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: ReportFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<production_reports::Model>, u64)> {
    let mut query = entity::ProductionReports::find();

    if let Some(report_type) = filter.report_type {
        query = query.filter(production_reports::Column::ReportType.eq(report_type));
    }
    if let Some(operator) = filter.operator_id {
        query = query.filter(production_reports::Column::OperatorId.eq(operator));
    }
    if let Some(start) = filter.start_date {
        query = query.filter(production_reports::Column::ReportDate.gte(start));
    }
    if let Some(end) = filter.end_date {
        query = query.filter(production_reports::Column::ReportDate.lte(end));
    }

    query = query.order_by_desc(production_reports::Column::ReportTime);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<production_reports::Model>> {
    Ok(entity::ProductionReports::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: production_reports::ActiveModel,
) -> Result<production_reports::Model> {
    Ok(entity::ProductionReports::insert(active)
        .exec_with_returning(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: production_reports::ActiveModel,
) -> Result<Option<production_reports::Model>> {
    active.id = Set(id);
    let updated = entity::ProductionReports::update(active)
        .exec_with_returning(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = entity::ProductionReports::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


