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


