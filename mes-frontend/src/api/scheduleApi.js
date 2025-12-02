import request from "./axiosInstance";

export const fetchScheduleTimeline = (params) =>
  request.get("/schedule/timeline", { params });

export const createSchedule = (data) =>
  request.post("/schedule", data);

export const updateSchedule = (id, data) =>
  request.put(`/schedule/${id}`, data);

export const deleteSchedule = (id) =>
  request.delete(`/schedule/${id}`);

export const runAutoSchedule = (data) =>
  request.post("/schedule/run", data);


