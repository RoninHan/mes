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


