use crate::api::ApiContext;
use crate::db::dao;
use crate::model::master_data::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::ActiveValue::Set;

pub fn router() -> axum::Router<ApiContext> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route(
            "/master/material-categories",
            get(list_categories).post(create_category),
        )
        .route(
            "/master/material-categories/:id",
            get(get_category).put(update_category).delete(delete_category),
        )
        .route(
            "/master/materials",
            get(list_materials).post(create_material),
        )
        .route(
            "/master/materials/:id",
            get(get_material).put(update_material).delete(delete_material),
        )
        .route(
            "/master/suppliers",
            get(list_suppliers).post(create_supplier),
        )
        .route(
            "/master/suppliers/:id",
            get(get_supplier).put(update_supplier).delete(delete_supplier),
        )
        .route(
            "/master/customers",
            get(list_customers).post(create_customer),
        )
        .route(
            "/master/customers/:id",
            get(get_customer).put(update_customer).delete(delete_customer),
        )
}

// ----- Material Categories -----

async fn list_categories(
    State(ctx): State<ApiContext>,
    Query(q): Query<CategoryQuery>,
) -> Result<Json<PageResult<CategoryDto>>, StatusCode> {
    let filter = dao::material_category_dao::CategoryFilter {
        parent_id: q.parent_id,
        status: q.status,
    };
    let (items, total) = dao::material_category_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| CategoryDto {
            id: m.id,
            category_code: m.category_code,
            category_name: m.category_name,
            parent_id: m.parent_id,
            category_level: m.category_level,
            sort_order: m.sort_order,
            status: m.status,
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_category(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<CategoryDto>, StatusCode> {
    let Some(m) = dao::material_category_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(CategoryDto {
        id: m.id,
        category_code: m.category_code,
        category_name: m.category_name,
        parent_id: m.parent_id,
        category_level: m.category_level,
        sort_order: m.sort_order,
        status: m.status,
        remark: m.remark,
    }))
}

async fn create_category(
    State(ctx): State<ApiContext>,
    Json(body): Json<CategoryPayload>,
) -> Result<Json<CategoryDto>, StatusCode> {
    let active = crate::db::entity::material_categories::ActiveModel {
        category_code: Set(body.category_code),
        category_name: Set(body.category_name),
        parent_id: Set(body.parent_id.unwrap_or(0)),
        category_level: Set(body.category_level.unwrap_or(1)),
        sort_order: Set(body.sort_order.unwrap_or(0)),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };

    let m = dao::material_category_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CategoryDto {
        id: m.id,
        category_code: m.category_code,
        category_name: m.category_name,
        parent_id: m.parent_id,
        category_level: m.category_level,
        sort_order: m.sort_order,
        status: m.status,
        remark: m.remark,
    }))
}

async fn update_category(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<CategoryPayload>,
) -> Result<Json<CategoryDto>, StatusCode> {
    let active = crate::db::entity::material_categories::ActiveModel {
        category_code: Set(body.category_code),
        category_name: Set(body.category_name),
        parent_id: Set(body.parent_id.unwrap_or(0)),
        category_level: Set(body.category_level.unwrap_or(1)),
        sort_order: Set(body.sort_order.unwrap_or(0)),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };

    let Some(m) = dao::material_category_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(CategoryDto {
        id: m.id,
        category_code: m.category_code,
        category_name: m.category_name,
        parent_id: m.parent_id,
        category_level: m.category_level,
        sort_order: m.sort_order,
        status: m.status,
        remark: m.remark,
    }))
}

async fn delete_category(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::material_category_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

// ----- Materials -----

async fn list_materials(
    State(ctx): State<ApiContext>,
    Query(q): Query<MaterialsQuery>,
) -> Result<Json<PageResult<MaterialDto>>, StatusCode> {
    let filter = dao::materials_dao::MaterialsFilter {
        category_id: q.category_id,
        material_type: q.material_type,
        keyword: q.keyword.clone(),
    };
    let (items, total) =
        dao::materials_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| MaterialDto {
            id: m.id,
            material_code: m.material_code,
            material_name: m.material_name,
            material_spec: m.material_spec,
            category_id: m.category_id,
            material_type: m.material_type,
            unit: m.unit,
            status: m.status,
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_material(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<MaterialDto>, StatusCode> {
    let Some(m) = dao::materials_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(MaterialDto {
        id: m.id,
        material_code: m.material_code,
        material_name: m.material_name,
        material_spec: m.material_spec,
        category_id: m.category_id,
        material_type: m.material_type,
        unit: m.unit,
        status: m.status,
        remark: m.remark,
    }))
}

async fn create_material(
    State(ctx): State<ApiContext>,
    Json(body): Json<MaterialPayload>,
) -> Result<Json<MaterialDto>, StatusCode> {
    let active = crate::db::entity::materials::ActiveModel {
        material_code: Set(body.material_code),
        material_name: Set(body.material_name),
        material_spec: Set(body.material_spec),
        material_model: Set(body.material_model),
        category_id: Set(body.category_id),
        material_type: Set(body.material_type),
        unit: Set(body.unit),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::materials_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(MaterialDto {
        id: m.id,
        material_code: m.material_code,
        material_name: m.material_name,
        material_spec: m.material_spec,
        category_id: m.category_id,
        material_type: m.material_type,
        unit: m.unit,
        status: m.status,
        remark: m.remark,
    }))
}

async fn update_material(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<MaterialPayload>,
) -> Result<Json<MaterialDto>, StatusCode> {
    let active = crate::db::entity::materials::ActiveModel {
        material_code: Set(body.material_code),
        material_name: Set(body.material_name),
        material_spec: Set(body.material_spec),
        material_model: Set(body.material_model),
        category_id: Set(body.category_id),
        material_type: Set(body.material_type),
        unit: Set(body.unit),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::materials_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(MaterialDto {
        id: m.id,
        material_code: m.material_code,
        material_name: m.material_name,
        material_spec: m.material_spec,
        category_id: m.category_id,
        material_type: m.material_type,
        unit: m.unit,
        status: m.status,
        remark: m.remark,
    }))
}

async fn delete_material(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::materials_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

// ----- Suppliers -----

async fn list_suppliers(
    State(ctx): State<ApiContext>,
    Query(q): Query<SupplierQuery>,
) -> Result<Json<PageResult<SupplierDto>>, StatusCode> {
    let filter = dao::suppliers_dao::SupplierFilter {
        supplier_type: q.supplier_type,
        status: q.status,
        keyword: q.keyword.clone(),
    };
    let (items, total) =
        dao::suppliers_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| SupplierDto {
            id: m.id,
            supplier_code: m.supplier_code,
            supplier_name: m.supplier_name,
            supplier_type: m.supplier_type,
            supplier_level: m.supplier_level,
            contact_person: m.contact_person,
            contact_phone: m.contact_phone,
            status: m.status,
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_supplier(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<SupplierDto>, StatusCode> {
    let Some(m) = dao::suppliers_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(SupplierDto {
        id: m.id,
        supplier_code: m.supplier_code,
        supplier_name: m.supplier_name,
        supplier_type: m.supplier_type,
        supplier_level: m.supplier_level,
        contact_person: m.contact_person,
        contact_phone: m.contact_phone,
        status: m.status,
        remark: m.remark,
    }))
}

async fn create_supplier(
    State(ctx): State<ApiContext>,
    Json(body): Json<SupplierPayload>,
) -> Result<Json<SupplierDto>, StatusCode> {
    let active = crate::db::entity::suppliers::ActiveModel {
        supplier_code: Set(body.supplier_code),
        supplier_name: Set(body.supplier_name),
        supplier_type: Set(body.supplier_type),
        supplier_level: Set(body.supplier_level),
        contact_person: Set(body.contact_person),
        contact_phone: Set(body.contact_phone),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::suppliers_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(SupplierDto {
        id: m.id,
        supplier_code: m.supplier_code,
        supplier_name: m.supplier_name,
        supplier_type: m.supplier_type,
        supplier_level: m.supplier_level,
        contact_person: m.contact_person,
        contact_phone: m.contact_phone,
        status: m.status,
        remark: m.remark,
    }))
}

async fn update_supplier(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<SupplierPayload>,
) -> Result<Json<SupplierDto>, StatusCode> {
    let active = crate::db::entity::suppliers::ActiveModel {
        supplier_code: Set(body.supplier_code),
        supplier_name: Set(body.supplier_name),
        supplier_type: Set(body.supplier_type),
        supplier_level: Set(body.supplier_level),
        contact_person: Set(body.contact_person),
        contact_phone: Set(body.contact_phone),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::suppliers_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(SupplierDto {
        id: m.id,
        supplier_code: m.supplier_code,
        supplier_name: m.supplier_name,
        supplier_type: m.supplier_type,
        supplier_level: m.supplier_level,
        contact_person: m.contact_person,
        contact_phone: m.contact_phone,
        status: m.status,
        remark: m.remark,
    }))
}

async fn delete_supplier(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::suppliers_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

// ----- Customers -----

async fn list_customers(
    State(ctx): State<ApiContext>,
    Query(q): Query<CustomerQuery>,
) -> Result<Json<PageResult<CustomerDto>>, StatusCode> {
    let filter = dao::customers_dao::CustomerFilter {
        customer_type: q.customer_type,
        status: q.status,
        keyword: q.keyword.clone(),
    };
    let (items, total) =
        dao::customers_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| CustomerDto {
            id: m.id,
            customer_code: m.customer_code,
            customer_name: m.customer_name,
            customer_type: m.customer_type,
            customer_level: m.customer_level,
            contact_person: m.contact_person,
            contact_phone: m.contact_phone,
            status: m.status,
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_customer(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<CustomerDto>, StatusCode> {
    let Some(m) = dao::customers_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(CustomerDto {
        id: m.id,
        customer_code: m.customer_code,
        customer_name: m.customer_name,
        customer_type: m.customer_type,
        customer_level: m.customer_level,
        contact_person: m.contact_person,
        contact_phone: m.contact_phone,
        status: m.status,
        remark: m.remark,
    }))
}

async fn create_customer(
    State(ctx): State<ApiContext>,
    Json(body): Json<CustomerPayload>,
) -> Result<Json<CustomerDto>, StatusCode> {
    let active = crate::db::entity::customers::ActiveModel {
        customer_code: Set(body.customer_code),
        customer_name: Set(body.customer_name),
        customer_type: Set(body.customer_type),
        customer_level: Set(body.customer_level),
        contact_person: Set(body.contact_person),
        contact_phone: Set(body.contact_phone),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::customers_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(CustomerDto {
        id: m.id,
        customer_code: m.customer_code,
        customer_name: m.customer_name,
        customer_type: m.customer_type,
        customer_level: m.customer_level,
        contact_person: m.contact_person,
        contact_phone: m.contact_phone,
        status: m.status,
        remark: m.remark,
    }))
}

async fn update_customer(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<CustomerPayload>,
) -> Result<Json<CustomerDto>, StatusCode> {
    let active = crate::db::entity::customers::ActiveModel {
        customer_code: Set(body.customer_code),
        customer_name: Set(body.customer_name),
        customer_type: Set(body.customer_type),
        customer_level: Set(body.customer_level),
        contact_person: Set(body.contact_person),
        contact_phone: Set(body.contact_phone),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::customers_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(CustomerDto {
        id: m.id,
        customer_code: m.customer_code,
        customer_name: m.customer_name,
        customer_type: m.customer_type,
        customer_level: m.customer_level,
        contact_person: m.contact_person,
        contact_phone: m.contact_phone,
        status: m.status,
        remark: m.remark,
    }))
}

async fn delete_customer(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::customers_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}


