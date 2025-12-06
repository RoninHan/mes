import axiosInstance from "./axiosInstance";

// Quality Inspection Tasks
export const fetchInspectionTasks = async (params = {}) => {
  const res = await axiosInstance.get("/quality/inspection-tasks", { params });
  return res.data;
};

export const getInspectionTask = async (id) => {
  const res = await axiosInstance.get(`/quality/inspection-tasks/${id}`);
  return res.data;
};

export const createInspectionTask = async (data) => {
  const res = await axiosInstance.post("/quality/inspection-tasks", data);
  return res.data;
};

export const updateInspectionTask = async (id, data) => {
  const res = await axiosInstance.put(`/quality/inspection-tasks/${id}`, data);
  return res.data;
};

export const deleteInspectionTask = async (id) => {
  await axiosInstance.delete(`/quality/inspection-tasks/${id}`);
};

// Quality Inspection Reports
export const fetchInspectionReports = async (params = {}) => {
  const res = await axiosInstance.get("/quality/inspection-reports", { params });
  return res.data;
};

export const getInspectionReport = async (id) => {
  const res = await axiosInstance.get(`/quality/inspection-reports/${id}`);
  return res.data;
};

export const createInspectionReport = async (data) => {
  const res = await axiosInstance.post("/quality/inspection-reports", data);
  return res.data;
};

export const updateInspectionReport = async (id, data) => {
  const res = await axiosInstance.put(`/quality/inspection-reports/${id}`, data);
  return res.data;
};

export const deleteInspectionReport = async (id) => {
  await axiosInstance.delete(`/quality/inspection-reports/${id}`);
};

// Nonconforming Products (NCR)
export const fetchNcr = async (params = {}) => {
  const res = await axiosInstance.get("/quality/ncr", { params });
  return res.data;
};

export const getNcr = async (id) => {
  const res = await axiosInstance.get(`/quality/ncr/${id}`);
  return res.data;
};

export const createNcr = async (data) => {
  const res = await axiosInstance.post("/quality/ncr", data);
  return res.data;
};

export const updateNcr = async (id, data) => {
  const res = await axiosInstance.put(`/quality/ncr/${id}`, data);
  return res.data;
};

export const deleteNcr = async (id) => {
  await axiosInstance.delete(`/quality/ncr/${id}`);
};

// Customer Complaints
export const fetchComplaints = async (params = {}) => {
  const res = await axiosInstance.get("/quality/complaints", { params });
  return res.data;
};

export const getComplaint = async (id) => {
  const res = await axiosInstance.get(`/quality/complaints/${id}`);
  return res.data;
};

export const createComplaint = async (data) => {
  const res = await axiosInstance.post("/quality/complaints", data);
  return res.data;
};

export const updateComplaint = async (id, data) => {
  const res = await axiosInstance.put(`/quality/complaints/${id}`, data);
  return res.data;
};

export const deleteComplaint = async (id) => {
  await axiosInstance.delete(`/quality/complaints/${id}`);
};

// Rework Orders
export const fetchReworkOrders = async (params = {}) => {
  const res = await axiosInstance.get("/quality/rework-orders", { params });
  return res.data;
};

export const getReworkOrder = async (id) => {
  const res = await axiosInstance.get(`/quality/rework-orders/${id}`);
  return res.data;
};

export const createReworkOrder = async (data) => {
  const res = await axiosInstance.post("/quality/rework-orders", data);
  return res.data;
};

export const updateReworkOrder = async (id, data) => {
  const res = await axiosInstance.put(`/quality/rework-orders/${id}`, data);
  return res.data;
};

export const deleteReworkOrder = async (id) => {
  await axiosInstance.delete(`/quality/rework-orders/${id}`);
};

// Measuring Equipment
export const fetchMeasuringEquipment = async (params = {}) => {
  const res = await axiosInstance.get("/quality/measuring-equipment", { params });
  return res.data;
};

export const getMeasuringEquipment = async (id) => {
  const res = await axiosInstance.get(`/quality/measuring-equipment/${id}`);
  return res.data;
};

export const createMeasuringEquipment = async (data) => {
  const res = await axiosInstance.post("/quality/measuring-equipment", data);
  return res.data;
};

export const updateMeasuringEquipment = async (id, data) => {
  const res = await axiosInstance.put(`/quality/measuring-equipment/${id}`, data);
  return res.data;
};

export const deleteMeasuringEquipment = async (id) => {
  await axiosInstance.delete(`/quality/measuring-equipment/${id}`);
};

// Supplier Quality Evaluations
export const fetchSupplierEvaluations = async (params = {}) => {
  const res = await axiosInstance.get("/quality/supplier-evaluations", { params });
  return res.data;
};

export const getSupplierEvaluation = async (id) => {
  const res = await axiosInstance.get(`/quality/supplier-evaluations/${id}`);
  return res.data;
};

export const createSupplierEvaluation = async (data) => {
  const res = await axiosInstance.post("/quality/supplier-evaluations", data);
  return res.data;
};

export const updateSupplierEvaluation = async (id, data) => {
  const res = await axiosInstance.put(`/quality/supplier-evaluations/${id}`, data);
  return res.data;
};

export const deleteSupplierEvaluation = async (id) => {
  await axiosInstance.delete(`/quality/supplier-evaluations/${id}`);
};

// Quality Traceability Records
export const fetchTraceabilityRecords = async (params = {}) => {
  const res = await axiosInstance.get("/quality/traceability", { params });
  return res.data;
};

export const getTraceabilityRecord = async (id) => {
  const res = await axiosInstance.get(`/quality/traceability/${id}`);
  return res.data;
};

export const createTraceabilityRecord = async (data) => {
  const res = await axiosInstance.post("/quality/traceability", data);
  return res.data;
};

export const updateTraceabilityRecord = async (id, data) => {
  const res = await axiosInstance.put(`/quality/traceability/${id}`, data);
  return res.data;
};

export const deleteTraceabilityRecord = async (id) => {
  await axiosInstance.delete(`/quality/traceability/${id}`);
};

// Quality Costs
export const fetchQualityCosts = async (params = {}) => {
  const res = await axiosInstance.get("/quality/costs", { params });
  return res.data;
};

export const getQualityCost = async (id) => {
  const res = await axiosInstance.get(`/quality/costs/${id}`);
  return res.data;
};

export const createQualityCost = async (data) => {
  const res = await axiosInstance.post("/quality/costs", data);
  return res.data;
};

export const updateQualityCost = async (id, data) => {
  const res = await axiosInstance.put(`/quality/costs/${id}`, data);
  return res.data;
};

export const deleteQualityCost = async (id) => {
  await axiosInstance.delete(`/quality/costs/${id}`);
};

// Quality KPI
export const fetchQualityKpi = async (params = {}) => {
  const res = await axiosInstance.get("/quality/kpi", { params });
  return res.data;
};

export const getQualityKpi = async (id) => {
  const res = await axiosInstance.get(`/quality/kpi/${id}`);
  return res.data;
};

export const createQualityKpi = async (data) => {
  const res = await axiosInstance.post("/quality/kpi", data);
  return res.data;
};

export const updateQualityKpi = async (id, data) => {
  const res = await axiosInstance.put(`/quality/kpi/${id}`, data);
  return res.data;
};

