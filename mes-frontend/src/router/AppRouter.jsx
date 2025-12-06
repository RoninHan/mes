import React from "react";
import { Layout, Menu } from "antd";
import { useRoutes, Navigate } from "react-router-dom";
import equipmentRoutes from "./equipmentRoutes";
import masterRoutes from "./masterRoutes";
import productionRoutes from "./productionRoutes";
import warehouseRoutes from "./warehouseRoutes";
import systemRoutes from "./systemRoutes";
import qualityRoutes from "./qualityRoutes";

const { Header, Sider, Content } = Layout;

const routes = [
  { path: "/", element: <Navigate to="/equipment/list" replace /> },
  ...equipmentRoutes,
  ...masterRoutes,
  ...productionRoutes,
  ...warehouseRoutes,
  ...systemRoutes,
  ...qualityRoutes
];

function RouterView() {
  return useRoutes(routes);
}

export default function AppRouter() {
  return (
    <Layout style={{ minHeight: "100vh" }}>
      <Header style={{ color: "#fff", fontSize: 18 }}>MES 系统</Header>
      <Layout>
        <Sider width={220} theme="light">
          <Menu
            mode="inline"
            items={[
              {
                key: "equipment",
                label: "设备管理",
                children: [
                  { key: "equipment-list", label: "设备台账", onClick: () => (window.location.href = "/equipment/list") },
                  { key: "equipment-status-log", label: "状态日志", onClick: () => (window.location.href = "/equipment/status-log") },
                  { key: "equipment-control", label: "设备控制", onClick: () => (window.location.href = "/equipment/control") },
                  { key: "equipment-maintenance-plans", label: "维护计划", onClick: () => (window.location.href = "/equipment/maintenance-plans") },
                  { key: "equipment-maintenance-tasks", label: "维护任务", onClick: () => (window.location.href = "/equipment/maintenance-tasks") },
                  { key: "equipment-fault-reports", label: "故障报修", onClick: () => (window.location.href = "/equipment/fault-reports") },
                  { key: "equipment-repair-orders", label: "维修工单", onClick: () => (window.location.href = "/equipment/repair-orders") },
                  { key: "equipment-inspections", label: "点检记录", onClick: () => (window.location.href = "/equipment/inspections") },
                  { key: "equipment-kpi", label: "设备KPI", onClick: () => (window.location.href = "/equipment/kpi") }
                ]
              },
              {
                key: "master",
                label: "主数据管理",
                children: [
                  { key: "master-categories", label: "物料分类", onClick: () => (window.location.href = "/master/material-categories") },
                  { key: "master-materials", label: "物料主数据", onClick: () => (window.location.href = "/master/materials") },
                  { key: "master-suppliers", label: "供应商", onClick: () => (window.location.href = "/master/suppliers") },
                  { key: "master-customers", label: "客户", onClick: () => (window.location.href = "/master/customers") },
                  { key: "master-workshops", label: "车间", onClick: () => (window.location.href = "/master/workshops") },
                  { key: "master-warehouses", label: "仓库", onClick: () => (window.location.href = "/master/warehouses") },
                  { key: "master-locations", label: "库位", onClick: () => (window.location.href = "/master/locations") },
                  { key: "master-boms", label: "BOM 管理", onClick: () => (window.location.href = "/master/boms") },
                  { key: "master-routes", label: "工艺路线", onClick: () => (window.location.href = "/master/routes") }
                ]
              },
              {
                key: "production",
                label: "生产管理",
                children: [
                  { key: "production-plans", label: "生产计划", onClick: () => (window.location.href = "/production/plans") },
                  { key: "production-orders", label: "生产订单", onClick: () => (window.location.href = "/production/orders") },
                  { key: "production-work-orders", label: "生产工单", onClick: () => (window.location.href = "/production/work-orders") },
                  { key: "production-reports", label: "生产报工", onClick: () => (window.location.href = "/production/reports") },
                  { key: "production-schedule", label: "生产排程", onClick: () => (window.location.href = "/production/schedule") },
                  { key: "production-material-requirements", label: "物料需求", onClick: () => (window.location.href = "/production/material-requirements") },
                  { key: "production-picking-orders", label: "领料单", onClick: () => (window.location.href = "/production/picking-orders") },
                  { key: "production-return-orders", label: "退料单", onClick: () => (window.location.href = "/production/return-orders") },
                  { key: "production-receipts", label: "完工入库", onClick: () => (window.location.href = "/production/receipts") }
                ]
              },
              {
                key: "warehouse",
                label: "仓储管理",
                children: [
                  { key: "warehouse-inventory", label: "库存总览", onClick: () => (window.location.href = "/warehouse/inventory") },
                  { key: "warehouse-inbound-orders", label: "入库单", onClick: () => (window.location.href = "/warehouse/inbound-orders") },
                  { key: "warehouse-outbound-orders", label: "出库单", onClick: () => (window.location.href = "/warehouse/outbound-orders") },
                  { key: "warehouse-transfer-orders", label: "调拨单", onClick: () => (window.location.href = "/warehouse/transfer-orders") },
                  { key: "warehouse-stock-count-orders", label: "盘点单", onClick: () => (window.location.href = "/warehouse/stock-count-orders") }
                ]
              },
              {
                key: "quality",
                label: "质量管理",
                children: [
                  { key: "quality-inspection-tasks", label: "质检任务", onClick: () => (window.location.href = "/quality/inspection-tasks") },
                  { key: "quality-inspection-reports", label: "质检报告", onClick: () => (window.location.href = "/quality/inspection-reports") },
                  { key: "quality-ncr", label: "不合格品（NCR）", onClick: () => (window.location.href = "/quality/ncr") },
                  { key: "quality-complaints", label: "客户投诉", onClick: () => (window.location.href = "/quality/complaints") },
                  { key: "quality-rework-orders", label: "返工单", onClick: () => (window.location.href = "/quality/rework-orders") },
                  { key: "quality-measuring-equipment", label: "测量设备", onClick: () => (window.location.href = "/quality/measuring-equipment") },
                  { key: "quality-supplier-evaluations", label: "供应商评估", onClick: () => (window.location.href = "/quality/supplier-evaluations") },
                  { key: "quality-traceability", label: "质量追溯", onClick: () => (window.location.href = "/quality/traceability") },
                  { key: "quality-costs", label: "质量成本", onClick: () => (window.location.href = "/quality/costs") },
                  { key: "quality-kpi", label: "质量KPI看板", onClick: () => (window.location.href = "/quality/kpi") }
                ]
              },
              {
                key: "system",
                label: "系统管理",
                children: [
                  { key: "system-user", label: "用户管理", onClick: () => (window.location.href = "/system/user") },
                  { key: "system-login-logs", label: "登录日志", onClick: () => (window.location.href = "/system/login-logs") },
                  { key: "system-operation-logs", label: "操作审计", onClick: () => (window.location.href = "/system/operation-logs") }
                ]
              }
            ]}
          />
        </Sider>
        <Content style={{ padding: 16 }}>
          <RouterView />
        </Content>
      </Layout>
    </Layout>
  );
}


