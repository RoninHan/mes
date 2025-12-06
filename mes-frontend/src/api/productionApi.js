import request from "./axiosInstance";

// Production Plans
export const fetchProductionPlans = (params) =>
  request.get("/production/plans", { params });
export const fetchProductionPlan = (id) =>
  request.get(`/production/plans/${id}`);
export const createProductionPlan = (data) =>
  request.post("/production/plans", data);
export const updateProductionPlan = (id, data) =>
  request.put(`/production/plans/${id}`, data);
export const deleteProductionPlan = (id) =>
  request.delete(`/production/plans/${id}`);

// Production Orders
export const fetchProductionOrders = (params) =>
  request.get("/production/orders", { params });
export const fetchProductionOrder = (id) =>
  request.get(`/production/orders/${id}`);
export const createProductionOrder = (data) =>
  request.post("/production/orders", data);
export const updateProductionOrder = (id, data) =>
  request.put(`/production/orders/${id}`, data);
export const deleteProductionOrder = (id) =>
  request.delete(`/production/orders/${id}`);

// Work Orders
export const fetchWorkOrders = (params) =>
  request.get("/production/work-orders", { params });
export const fetchWorkOrder = (id) =>
  request.get(`/production/work-orders/${id}`);
export const createWorkOrder = (data) =>
  request.post("/production/work-orders", data);
export const updateWorkOrder = (id, data) =>
  request.put(`/production/work-orders/${id}`, data);
export const deleteWorkOrder = (id) =>
  request.delete(`/production/work-orders/${id}`);

// Production Reports
export const fetchProductionReports = (params) =>
  request.get("/production/reports", { params });
export const fetchProductionReport = (id) =>
  request.get(`/production/reports/${id}`);
export const createProductionReport = (data) =>
  request.post("/production/reports", data);
export const updateProductionReport = (id, data) =>
  request.put(`/production/reports/${id}`, data);
export const deleteProductionReport = (id) =>
  request.delete(`/production/reports/${id}`);

// Material Requirements
export const fetchMaterialRequirements = (params) =>
  request.get("/production/material-requirements", { params });
export const createMaterialRequirement = (data) =>
  request.post("/production/material-requirements", data);
export const updateMaterialRequirement = (id, data) =>
  request.put(`/production/material-requirements/${id}`, data);
export const deleteMaterialRequirement = (id) =>
  request.delete(`/production/material-requirements/${id}`);

// Picking Orders
export const fetchPickingOrders = (params) =>
  request.get("/production/picking-orders", { params });
export const fetchPickingOrder = (id) =>
  request.get(`/production/picking-orders/${id}`);
export const createPickingOrder = (data) =>
  request.post("/production/picking-orders", data);
export const updatePickingOrder = (id, data) =>
  request.put(`/production/picking-orders/${id}`, data);
export const deletePickingOrder = (id) =>
  request.delete(`/production/picking-orders/${id}`);

// Return Orders
export const fetchReturnOrders = (params) =>
  request.get("/production/return-orders", { params });
export const fetchReturnOrder = (id) =>
  request.get(`/production/return-orders/${id}`);
export const createReturnOrder = (data) =>
  request.post("/production/return-orders", data);
export const updateReturnOrder = (id, data) =>
  request.put(`/production/return-orders/${id}`, data);
export const deleteReturnOrder = (id) =>
  request.delete(`/production/return-orders/${id}`);

// Production Receipts (完工入库)
export const fetchProductionReceipts = (params) =>
  request.get("/production/receipts", { params });
export const fetchProductionReceipt = (id) =>
  request.get(`/production/receipts/${id}`);
export const createProductionReceipt = (data) =>
  request.post("/production/receipts", data);
export const updateProductionReceipt = (id, data) =>
  request.put(`/production/receipts/${id}`, data);
export const deleteProductionReceipt = (id) =>
  request.delete(`/production/receipts/${id}`);


