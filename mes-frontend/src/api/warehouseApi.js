import request from "./axiosInstance";

// Inventory
export const fetchInventory = (params) =>
  request.get("/warehouse/inventory", { params });

export const fetchInventoryItem = (id) =>
  request.get(`/warehouse/inventory/${id}`);

// Inbound orders
export const fetchInboundOrders = (params) =>
  request.get("/warehouse/inbound-orders", { params });

export const fetchInboundOrder = (id) =>
  request.get(`/warehouse/inbound-orders/${id}`);

export const createInboundOrder = (data) =>
  request.post("/warehouse/inbound-orders", data);

export const updateInboundOrder = (id, data) =>
  request.put(`/warehouse/inbound-orders/${id}`, data);

export const deleteInboundOrder = (id) =>
  request.delete(`/warehouse/inbound-orders/${id}`);


