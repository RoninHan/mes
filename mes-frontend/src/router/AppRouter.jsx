import React from "react";
import { Layout, Menu } from "antd";
import { useRoutes, Navigate } from "react-router-dom";
import equipmentRoutes from "./equipmentRoutes";
import masterRoutes from "./masterRoutes";
import productionRoutes from "./productionRoutes";
import warehouseRoutes from "./warehouseRoutes";
import systemRoutes from "./systemRoutes";

const { Header, Sider, Content } = Layout;

const routes = [
  { path: "/", element: <Navigate to="/equipment/list" replace /> },
  ...equipmentRoutes,
  ...masterRoutes,
  ...productionRoutes,
  ...warehouseRoutes,
  ...systemRoutes
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
                  { key: "equipment-control", label: "设备控制", onClick: () => (window.location.href = "/equipment/control") }
                ]
              },
              {
                key: "master",
                label: "主数据管理",
                children: [
                  { key: "master-categories", label: "物料分类", onClick: () => (window.location.href = "/master/material-categories") },
                  { key: "master-materials", label: "物料主数据", onClick: () => (window.location.href = "/master/materials") },
                  { key: "master-suppliers", label: "供应商", onClick: () => (window.location.href = "/master/suppliers") },
                  { key: "master-customers", label: "客户", onClick: () => (window.location.href = "/master/customers") }
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
                  { key: "production-schedule", label: "生产排程", onClick: () => (window.location.href = "/production/schedule") }
                ]
              },
              {
                key: "warehouse",
                label: "仓储管理",
                children: [
                  { key: "warehouse-inventory", label: "库存总览", onClick: () => (window.location.href = "/warehouse/inventory") },
                  { key: "warehouse-inbound-orders", label: "入库单", onClick: () => (window.location.href = "/warehouse/inbound-orders") }
                ]
              },
              {
                key: "system",
                label: "系统管理",
                children: [
                  { key: "system-user", label: "用户管理", onClick: () => (window.location.href = "/system/user") }
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


