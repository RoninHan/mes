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

// Outbound orders
export const fetchOutboundOrders = (params) =>
  request.get("/warehouse/outbound-orders", { params });

export const fetchOutboundOrder = (id) =>
  request.get(`/warehouse/outbound-orders/${id}`);

export const createOutboundOrder = (data) =>
  request.post("/warehouse/outbound-orders", data);

export const updateOutboundOrder = (id, data) =>
  request.put(`/warehouse/outbound-orders/${id}`, data);

export const deleteOutboundOrder = (id) =>
  request.delete(`/warehouse/outbound-orders/${id}`);

// Transfer orders
export const fetchTransferOrders = (params) =>
  request.get("/warehouse/transfer-orders", { params });

export const fetchTransferOrder = (id) =>
  request.get(`/warehouse/transfer-orders/${id}`);

export const createTransferOrder = (data) =>
  request.post("/warehouse/transfer-orders", data);

export const updateTransferOrder = (id, data) =>
  request.put(`/warehouse/transfer-orders/${id}`, data);

export const deleteTransferOrder = (id) =>
  request.delete(`/warehouse/transfer-orders/${id}`);

// Stock count orders
export const fetchStockCountOrders = (params) =>
  request.get("/warehouse/stock-count-orders", { params });

export const fetchStockCountOrder = (id) =>
  request.get(`/warehouse/stock-count-orders/${id}`);

export const createStockCountOrder = (data) =>
  request.post("/warehouse/stock-count-orders", data);

export const updateStockCountOrder = (id, data) =>
  request.put(`/warehouse/stock-count-orders/${id}`, data);

export const deleteStockCountOrder = (id) =>
  request.delete(`/warehouse/stock-count-orders/${id}`);


