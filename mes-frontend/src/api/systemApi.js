// 注意：用户管理已迁移至 SSO，此文件保留审计日志和关联应用地址 API。
import request from "./axiosInstance";

// ── 关联应用地址配置 ───────────────────────────────────────────────────────────

/** 获取关联系统地址（ERP / SSO URL） */
export const fetchAppLinks = () =>
  request.get("/system/app-links");

/** 更新关联系统地址（仅管理员） */
export const updateAppLinks = (data) =>
  request.put("/system/app-links", data);

// ── 登录日志 ──────────────────────────────────────────────────────────────────

export const fetchLoginLogs = (params) =>
  request.get("/system/login-logs", { params });

// ── 操作审计日志 ──────────────────────────────────────────────────────────────

export const fetchOperationLogs = (params) =>
  request.get("/system/operation-logs", { params });
