import request from "./axiosInstance";

// Material Categories
export const fetchMaterialCategories = (params) =>
  request.get("/master/material-categories", { params });

export const fetchMaterialCategory = (id) =>
  request.get(`/master/material-categories/${id}`);

export const createMaterialCategory = (data) =>
  request.post("/master/material-categories", data);

export const updateMaterialCategory = (id, data) =>
  request.put(`/master/material-categories/${id}`, data);

export const deleteMaterialCategory = (id) =>
  request.delete(`/master/material-categories/${id}`);

// Materials
export const fetchMaterials = (params) =>
  request.get("/master/materials", { params });

export const fetchMaterial = (id) =>
  request.get(`/master/materials/${id}`);

export const createMaterial = (data) =>
  request.post("/master/materials", data);

export const updateMaterial = (id, data) =>
  request.put(`/master/materials/${id}`, data);

export const deleteMaterial = (id) =>
  request.delete(`/master/materials/${id}`);

// Suppliers
export const fetchSuppliers = (params) =>
  request.get("/master/suppliers", { params });

export const fetchSupplier = (id) =>
  request.get(`/master/suppliers/${id}`);

export const createSupplier = (data) =>
  request.post("/master/suppliers", data);

export const updateSupplier = (id, data) =>
  request.put(`/master/suppliers/${id}`, data);

export const deleteSupplier = (id) =>
  request.delete(`/master/suppliers/${id}`);

// Customers
export const fetchCustomers = (params) =>
  request.get("/master/customers", { params });

export const fetchCustomer = (id) =>
  request.get(`/master/customers/${id}`);

export const createCustomer = (data) =>
  request.post("/master/customers", data);

export const updateCustomer = (id, data) =>
  request.put(`/master/customers/${id}`, data);

export const deleteCustomer = (id) =>
  request.delete(`/master/customers/${id}`);

// Workshops
export const fetchWorkshops = (params) =>
  request.get("/master/workshops", { params });

export const createWorkshop = (data) =>
  request.post("/master/workshops", data);

export const updateWorkshop = (id, data) =>
  request.put(`/master/workshops/${id}`, data);

export const deleteWorkshop = (id) =>
  request.delete(`/master/workshops/${id}`);

// Warehouses
export const fetchWarehouses = (params) =>
  request.get("/master/warehouses", { params });

export const createWarehouse = (data) =>
  request.post("/master/warehouses", data);

export const updateWarehouse = (id, data) =>
  request.put(`/master/warehouses/${id}`, data);

export const deleteWarehouse = (id) =>
  request.delete(`/master/warehouses/${id}`);

// Locations
export const fetchLocations = (params) =>
  request.get("/master/locations", { params });

export const createLocation = (data) =>
  request.post("/master/locations", data);

export const updateLocation = (id, data) =>
  request.put(`/master/locations/${id}`, data);

export const deleteLocation = (id) =>
  request.delete(`/master/locations/${id}`);

// BOM
export const fetchBoms = (params) =>
  request.get("/master/boms", { params });

export const createBom = (data) =>
  request.post("/master/boms", data);

export const updateBom = (id, data) =>
  request.put(`/master/boms/${id}`, data);

export const deleteBom = (id) =>
  request.delete(`/master/boms/${id}`);

// Process Routes
export const fetchProcessRoutes = (params) =>
  request.get("/master/routes", { params });

export const createProcessRoute = (data) =>
  request.post("/master/routes", data);

export const updateProcessRoute = (id, data) =>
  request.put(`/master/routes/${id}`, data);

export const deleteProcessRoute = (id) =>
  request.delete(`/master/routes/${id}`);


