use crate::db::entity::{self, production_plans, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug)]
pub struct PlanFilter {
    pub plan_status: Option<i8>,
    pub plan_type: Option<i8>,
    pub keyword: Option<String>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: PlanFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<production_plans::Model>, u64)> {
    let mut query = production_plans::Entity::find();

    if let Some(status) = filter.plan_status {
        query = query.filter(production_plans::Column::PlanStatus.eq(status));
    }
    if let Some(plan_type) = filter.plan_type {
        query = query.filter(production_plans::Column::PlanType.eq(plan_type));
    }
    if let Some(keyword) = filter.keyword {
        let like = format!("%{}%", keyword);
        query = query.filter(
            production_plans::Column::PlanName
                .like(like.clone())
                .or(production_plans::Column::PlanNo.like(like)),
        );
    }

    query = query.order_by_desc(production_plans::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i64) -> Result<Option<production_plans::Model>> {
    Ok(production_plans::Entity::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: production_plans::ActiveModel,
) -> Result<production_plans::Model> {
    Ok(production_plans::Entity::insert(active)
        .exec(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: production_plans::ActiveModel,
) -> Result<Option<production_plans::Model>> {
    active.id = Set(id);
    let updated = production_plans::Entity::update(active)
        .exec(conn)
        .await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = production_plans::Entity::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}


