import request from "./axiosInstance";

export const fetchEquipmentList = (params) =>
  request.get("/equipment", { params });

export const fetchEquipmentDetail = (id) =>
  request.get(`/equipment/${id}`);

export const createEquipment = (data) =>
  request.post("/equipment", data);

export const updateEquipment = (id, data) =>
  request.put(`/equipment/${id}`, data);

export const deleteEquipment = (id) =>
  request.delete(`/equipment/${id}`);

export const fetchMqttConfig = (id) =>
  request.get(`/equipment/${id}/mqtt-config`);

export const saveMqttConfig = (id, data) =>
  request.post(`/equipment/${id}/mqtt-config`, data);

export const fetchEquipmentStatus = (id) =>
  request.get(`/equipment/${id}/status`);

export const fetchStatusLog = (params) =>
  request.get("/equipment/status-log", { params });

export const sendControlCommand = (id, data) =>
  request.post(`/equipment/${id}/control`, data);

// Maintenance plans
export const fetchMaintenancePlans = (params) =>
  request.get("/equipment/maintenance-plans", { params });
export const createMaintenancePlan = (data) =>
  request.post("/equipment/maintenance-plans", data);
export const updateMaintenancePlan = (id, data) =>
  request.put(`/equipment/maintenance-plans/${id}`, data);
export const deleteMaintenancePlan = (id) =>
  request.delete(`/equipment/maintenance-plans/${id}`);

// Maintenance tasks
export const fetchMaintenanceTasks = (params) =>
  request.get("/equipment/maintenance-tasks", { params });
export const createMaintenanceTask = (data) =>
  request.post("/equipment/maintenance-tasks", data);
export const updateMaintenanceTask = (id, data) =>
  request.put(`/equipment/maintenance-tasks/${id}`, data);
export const deleteMaintenanceTask = (id) =>
  request.delete(`/equipment/maintenance-tasks/${id}`);

// Fault reports
export const fetchFaultReports = (params) =>
  request.get("/equipment/fault-reports", { params });
export const createFaultReport = (data) =>
  request.post("/equipment/fault-reports", data);
export const updateFaultReport = (id, data) =>
  request.put(`/equipment/fault-reports/${id}`, data);
export const deleteFaultReport = (id) =>
  request.delete(`/equipment/fault-reports/${id}`);

// Repair orders
export const fetchRepairOrders = (params) =>
  request.get("/equipment/repair-orders", { params });
export const createRepairOrder = (data) =>
  request.post("/equipment/repair-orders", data);
export const updateRepairOrder = (id, data) =>
  request.put(`/equipment/repair-orders/${id}`, data);
export const deleteRepairOrder = (id) =>
  request.delete(`/equipment/repair-orders/${id}`);

// Inspections
export const fetchInspections = (params) =>
  request.get("/equipment/inspections", { params });
export const createInspection = (data) =>
  request.post("/equipment/inspections", data);
export const updateInspection = (id, data) =>
  request.put(`/equipment/inspections/${id}`, data);
export const deleteInspection = (id) =>
  request.delete(`/equipment/inspections/${id}`);

// Equipment KPI
export const fetchEquipmentKpi = (params) =>
  request.get("/equipment/kpi", { params });


