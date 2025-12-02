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


