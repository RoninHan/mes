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
        .route(
            "/master/workshops",
            get(list_workshops).post(create_workshop),
        )
        .route(
            "/master/workshops/:id",
            get(get_workshop).put(update_workshop).delete(delete_workshop),
        )
        .route(
            "/master/warehouses",
            get(list_warehouses).post(create_warehouse),
        )
        .route(
            "/master/warehouses/:id",
            get(get_warehouse).put(update_warehouse).delete(delete_warehouse),
        )
        .route(
            "/master/locations",
            get(list_locations).post(create_location),
        )
        .route(
            "/master/locations/:id",
            get(get_location).put(update_location).delete(delete_location),
        )
        .route("/master/boms", get(list_boms).post(create_bom))
        .route(
            "/master/boms/:id",
            get(get_bom).put(update_bom).delete(delete_bom),
        )
        .route(
            "/master/routes",
            get(list_process_routes).post(create_process_route),
        )
        .route(
            "/master/routes/:id",
            get(get_process_route)
                .put(update_process_route)
                .delete(delete_process_route),
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

// ----- Workshops -----

async fn list_workshops(
    State(ctx): State<ApiContext>,
    Query(q): Query<WorkshopQuery>,
) -> Result<Json<PageResult<WorkshopDto>>, StatusCode> {
    let filter = dao::workshop_dao::WorkshopFilter {
        status: q.status,
        keyword: q.keyword.clone(),
    };
    let (items, total) =
        dao::workshop_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| WorkshopDto {
            id: m.id,
            workshop_code: m.workshop_code,
            workshop_name: m.workshop_name,
            workshop_type: m.workshop_type,
            manager_id: m.manager_id,
            status: m.status,
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_workshop(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<WorkshopDto>, StatusCode> {
    let Some(m) = dao::workshop_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(WorkshopDto {
        id: m.id,
        workshop_code: m.workshop_code,
        workshop_name: m.workshop_name,
        workshop_type: m.workshop_type,
        manager_id: m.manager_id,
        status: m.status,
        remark: m.remark,
    }))
}

async fn create_workshop(
    State(ctx): State<ApiContext>,
    Json(body): Json<WorkshopPayload>,
) -> Result<Json<WorkshopDto>, StatusCode> {
    use crate::db::entity::workshops;
    let active = workshops::ActiveModel {
        workshop_code: Set(body.workshop_code),
        workshop_name: Set(body.workshop_name),
        workshop_type: Set(body.workshop_type),
        manager_id: Set(body.manager_id),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::workshop_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(WorkshopDto {
        id: m.id,
        workshop_code: m.workshop_code,
        workshop_name: m.workshop_name,
        workshop_type: m.workshop_type,
        manager_id: m.manager_id,
        status: m.status,
        remark: m.remark,
    }))
}

async fn update_workshop(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<WorkshopPayload>,
) -> Result<Json<WorkshopDto>, StatusCode> {
    use crate::db::entity::workshops;
    let active = workshops::ActiveModel {
        workshop_code: Set(body.workshop_code),
        workshop_name: Set(body.workshop_name),
        workshop_type: Set(body.workshop_type),
        manager_id: Set(body.manager_id),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::workshop_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(WorkshopDto {
        id: m.id,
        workshop_code: m.workshop_code,
        workshop_name: m.workshop_name,
        workshop_type: m.workshop_type,
        manager_id: m.manager_id,
        status: m.status,
        remark: m.remark,
    }))
}

async fn delete_workshop(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::workshop_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;
    Ok(StatusCode::NO_CONTENT)
}

// ----- Warehouses -----

async fn list_warehouses(
    State(ctx): State<ApiContext>,
    Query(q): Query<WarehouseQuery>,
) -> Result<Json<PageResult<WarehouseDto>>, StatusCode> {
    let filter = dao::warehouse_dao::WarehouseFilter {
        warehouse_type: q.warehouse_type,
        status: q.status,
        keyword: q.keyword.clone(),
    };
    let (items, total) =
        dao::warehouse_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| WarehouseDto {
            id: m.id,
            warehouse_code: m.warehouse_code,
            warehouse_name: m.warehouse_name,
            warehouse_type: m.warehouse_type,
            location: m.location,
            status: m.status,
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_warehouse(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<WarehouseDto>, StatusCode> {
    let Some(m) = dao::warehouse_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(WarehouseDto {
        id: m.id,
        warehouse_code: m.warehouse_code,
        warehouse_name: m.warehouse_name,
        warehouse_type: m.warehouse_type,
        location: m.location,
        status: m.status,
        remark: m.remark,
    }))
}

async fn create_warehouse(
    State(ctx): State<ApiContext>,
    Json(body): Json<WarehousePayload>,
) -> Result<Json<WarehouseDto>, StatusCode> {
    use crate::db::entity::warehouses;
    let active = warehouses::ActiveModel {
        warehouse_code: Set(body.warehouse_code),
        warehouse_name: Set(body.warehouse_name),
        warehouse_type: Set(body.warehouse_type),
        location: Set(body.location),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::warehouse_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(WarehouseDto {
        id: m.id,
        warehouse_code: m.warehouse_code,
        warehouse_name: m.warehouse_name,
        warehouse_type: m.warehouse_type,
        location: m.location,
        status: m.status,
        remark: m.remark,
    }))
}

async fn update_warehouse(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<WarehousePayload>,
) -> Result<Json<WarehouseDto>, StatusCode> {
    use crate::db::entity::warehouses;
    let active = warehouses::ActiveModel {
        warehouse_code: Set(body.warehouse_code),
        warehouse_name: Set(body.warehouse_name),
        warehouse_type: Set(body.warehouse_type),
        location: Set(body.location),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::warehouse_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(WarehouseDto {
        id: m.id,
        warehouse_code: m.warehouse_code,
        warehouse_name: m.warehouse_name,
        warehouse_type: m.warehouse_type,
        location: m.location,
        status: m.status,
        remark: m.remark,
    }))
}

async fn delete_warehouse(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::warehouse_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;
    Ok(StatusCode::NO_CONTENT)
}

// ----- Locations -----

async fn list_locations(
    State(ctx): State<ApiContext>,
    Query(q): Query<LocationQuery>,
) -> Result<Json<PageResult<LocationDto>>, StatusCode> {
    let filter = dao::location_dao::LocationFilter {
        warehouse_id: q.warehouse_id,
        status: q.status,
        keyword: q.keyword.clone(),
    };
    let (items, total) =
        dao::location_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| LocationDto {
            id: m.id,
            warehouse_id: m.warehouse_id,
            location_code: m.location_code,
            location_name: m.location_name,
            location_type: m.location_type,
            status: m.status,
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_location(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<LocationDto>, StatusCode> {
    let Some(m) = dao::location_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(LocationDto {
        id: m.id,
        warehouse_id: m.warehouse_id,
        location_code: m.location_code,
        location_name: m.location_name,
        location_type: m.location_type,
        status: m.status,
        remark: m.remark,
    }))
}

async fn create_location(
    State(ctx): State<ApiContext>,
    Json(body): Json<LocationPayload>,
) -> Result<Json<LocationDto>, StatusCode> {
    use crate::db::entity::locations;
    let active = locations::ActiveModel {
        warehouse_id: Set(body.warehouse_id),
        location_code: Set(body.location_code),
        location_name: Set(body.location_name),
        location_type: Set(body.location_type),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::location_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(LocationDto {
        id: m.id,
        warehouse_id: m.warehouse_id,
        location_code: m.location_code,
        location_name: m.location_name,
        location_type: m.location_type,
        status: m.status,
        remark: m.remark,
    }))
}

async fn update_location(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<LocationPayload>,
) -> Result<Json<LocationDto>, StatusCode> {
    use crate::db::entity::locations;
    let active = locations::ActiveModel {
        warehouse_id: Set(body.warehouse_id),
        location_code: Set(body.location_code),
        location_name: Set(body.location_name),
        location_type: Set(body.location_type),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::location_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(LocationDto {
        id: m.id,
        warehouse_id: m.warehouse_id,
        location_code: m.location_code,
        location_name: m.location_name,
        location_type: m.location_type,
        status: m.status,
        remark: m.remark,
    }))
}

async fn delete_location(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::location_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;
    Ok(StatusCode::NO_CONTENT)
}

// ----- BOM -----

async fn list_boms(
    State(ctx): State<ApiContext>,
    Query(q): Query<BomQuery>,
) -> Result<Json<PageResult<BomDto>>, StatusCode> {
    let filter = dao::bom_dao::BomFilter {
        material_id: q.material_id,
        status: q.status,
        is_default: q.is_default,
    };
    let (items, total) =
        dao::bom_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| BomDto {
            id: m.id,
            material_id: m.material_id,
            bom_code: m.bom_code,
            version: m.version,
            bom_type: m.bom_type,
            is_default: m.is_default,
            status: m.status,
            items: m.items.clone(),
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_bom(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<BomDto>, StatusCode> {
    let Some(m) = dao::bom_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(BomDto {
        id: m.id,
        material_id: m.material_id,
        bom_code: m.bom_code,
        version: m.version,
        bom_type: m.bom_type,
        is_default: m.is_default,
        status: m.status,
        items: m.items,
        remark: m.remark,
    }))
}

async fn create_bom(
    State(ctx): State<ApiContext>,
    Json(body): Json<BomPayload>,
) -> Result<Json<BomDto>, StatusCode> {
    use crate::db::entity::boms;
    let BomPayload {
        material_id,
        bom_code,
        version,
        bom_type,
        is_default,
        status,
        items,
        remark,
    } = body;
    let active = boms::ActiveModel {
        material_id: Set(material_id),
        bom_code: Set(bom_code),
        version: Set(version),
        bom_type: Set(bom_type),
        is_default: Set(is_default.unwrap_or(0)),
        status: Set(status.unwrap_or(1)),
        items: Set(items.clone()),
        remark: Set(remark),
        ..Default::default()
    };
    let m = dao::bom_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(BomDto {
        id: m.id,
        material_id: m.material_id,
        bom_code: m.bom_code,
        version: m.version,
        bom_type: m.bom_type,
        is_default: m.is_default,
        status: m.status,
        items: m.items,
        remark: m.remark,
    }))
}

async fn update_bom(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<BomPayload>,
) -> Result<Json<BomDto>, StatusCode> {
    use crate::db::entity::boms;
    let BomPayload {
        material_id,
        bom_code,
        version,
        bom_type,
        is_default,
        status,
        items,
        remark,
    } = body;
    let active = boms::ActiveModel {
        material_id: Set(material_id),
        bom_code: Set(bom_code),
        version: Set(version),
        bom_type: Set(bom_type),
        is_default: Set(is_default.unwrap_or(0)),
        status: Set(status.unwrap_or(1)),
        items: Set(items.clone()),
        remark: Set(remark),
        ..Default::default()
    };
    let Some(m) = dao::bom_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(BomDto {
        id: m.id,
        material_id: m.material_id,
        bom_code: m.bom_code,
        version: m.version,
        bom_type: m.bom_type,
        is_default: m.is_default,
        status: m.status,
        items: m.items,
        remark: m.remark,
    }))
}

async fn delete_bom(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::bom_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;
    Ok(StatusCode::NO_CONTENT)
}

// ----- Process Routes -----

async fn list_process_routes(
    State(ctx): State<ApiContext>,
    Query(q): Query<ProcessRouteQuery>,
) -> Result<Json<PageResult<ProcessRouteDto>>, StatusCode> {
    let filter = dao::process_route_dao::ProcessRouteFilter {
        material_id: q.material_id,
        status: q.status,
        is_default: q.is_default,
    };
    let (items, total) = dao::process_route_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| ProcessRouteDto {
            id: m.id,
            material_id: m.material_id,
            route_code: m.route_code,
            route_name: m.route_name,
            version: m.version,
            is_default: m.is_default,
            status: m.status,
            operations: m.operations.clone(),
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_process_route(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<ProcessRouteDto>, StatusCode> {
    let Some(m) = dao::process_route_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(ProcessRouteDto {
        id: m.id,
        material_id: m.material_id,
        route_code: m.route_code,
        route_name: m.route_name,
        version: m.version,
        is_default: m.is_default,
        status: m.status,
        operations: m.operations,
        remark: m.remark,
    }))
}

async fn create_process_route(
    State(ctx): State<ApiContext>,
    Json(body): Json<ProcessRoutePayload>,
) -> Result<Json<ProcessRouteDto>, StatusCode> {
    use crate::db::entity::process_routes;
    let ProcessRoutePayload {
        material_id,
        route_code,
        route_name,
        version,
        is_default,
        status,
        operations,
        remark,
    } = body;
    let active = process_routes::ActiveModel {
        material_id: Set(material_id),
        route_code: Set(route_code),
        route_name: Set(route_name),
        version: Set(version),
        is_default: Set(is_default.unwrap_or(0)),
        status: Set(status.unwrap_or(1)),
        operations: Set(operations.clone()),
        remark: Set(remark),
        ..Default::default()
    };
    let m = dao::process_route_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ProcessRouteDto {
        id: m.id,
        material_id: m.material_id,
        route_code: m.route_code,
        route_name: m.route_name,
        version: m.version,
        is_default: m.is_default,
        status: m.status,
        operations: m.operations,
        remark: m.remark,
    }))
}

async fn update_process_route(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<ProcessRoutePayload>,
) -> Result<Json<ProcessRouteDto>, StatusCode> {
    use crate::db::entity::process_routes;
    let ProcessRoutePayload {
        material_id,
        route_code,
        route_name,
        version,
        is_default,
        status,
        operations,
        remark,
    } = body;
    let active = process_routes::ActiveModel {
        material_id: Set(material_id),
        route_code: Set(route_code),
        route_name: Set(route_name),
        version: Set(version),
        is_default: Set(is_default.unwrap_or(0)),
        status: Set(status.unwrap_or(1)),
        operations: Set(operations.clone()),
        remark: Set(remark),
        ..Default::default()
    };
    let Some(m) = dao::process_route_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(ProcessRouteDto {
        id: m.id,
        material_id: m.material_id,
        route_code: m.route_code,
        route_name: m.route_name,
        version: m.version,
        is_default: m.is_default,
        status: m.status,
        operations: m.operations,
        remark: m.remark,
    }))
}

async fn delete_process_route(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::process_route_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;
    Ok(StatusCode::NO_CONTENT)
}



