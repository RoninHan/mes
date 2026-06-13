use crate::db::entity::{self, material_requirements, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};

#[derive(Debug)]
pub struct MaterialRequirementFilter {
    pub production_order_id: Option<i64>,
    pub material_id: Option<i64>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: MaterialRequirementFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<material_requirements::Model>, u64)> {
    let mut query = material_requirements::Entity::find()
        .filter(material_requirements::Column::IsDeleted.eq(0));

    if let Some(po) = filter.production_order_id {
        query = query.filter(material_requirements::Column::ProductionOrderId.eq(po));
    }
    if let Some(m) = filter.material_id {
        query = query.filter(material_requirements::Column::MaterialId.eq(m));
    }

    query = query.order_by_desc(material_requirements::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok((items, total))
}

pub async fn get_by_id(
    conn: ConnRef<'_>,
    id: i64,
) -> Result<Option<material_requirements::Model>> {
    Ok(material_requirements::Entity::find_by_id(id)
        .filter(material_requirements::Column::IsDeleted.eq(0))
        .one(conn)
        .await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    active: material_requirements::ActiveModel,
) -> Result<material_requirements::Model> {
    Ok(material_requirements::Entity::insert(active)
        .exec(conn)
        .await?)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i64,
    mut active: material_requirements::ActiveModel,
) -> Result<Option<material_requirements::Model>> {
    active.id = Set(id);
    Ok(Some(
        material_requirements::Entity::update(active)
            .exec(conn)
            .await?,
    ))
}

pub async fn delete(conn: ConnRef<'_>, id: i64) -> Result<u64> {
    let res = material_requirements::Entity::update_many()
        .col_expr(
            material_requirements::Column::IsDeleted,
            sea_orm_migration::sea_query::Expr::value(1),
        )
        .filter(material_requirements::Column::Id.eq(id))
        .exec(conn)
        .await?;
    Ok(res.rows_affected)
}


