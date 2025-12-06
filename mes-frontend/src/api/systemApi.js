import request from "./axiosInstance";

export const fetchUsers = (params) =>
  request.get("/system/users", { params });

export const fetchUser = (id) =>
  request.get(`/system/users/${id}`);

export const createUser = (data) =>
  request.post("/system/users", data);

export const updateUser = (id, data) =>
  request.put(`/system/users/${id}`, data);

export const deleteUser = (id) =>
  request.delete(`/system/users/${id}`);

// Login logs
export const fetchLoginLogs = (params) =>
  request.get("/system/login-logs", { params });

// Operation logs
export const fetchOperationLogs = (params) =>
  request.get("/system/operation-logs", { params });


