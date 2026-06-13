import React, { useState, useEffect, useRef } from "react";
import { Layout, Menu, Dropdown, Button, Select, Space, Avatar, message, Tooltip } from "antd";
import { fetchAppLinks } from "../api/systemApi";
import { useRoutes, Navigate, useLocation } from "react-router-dom";
import {
  UserOutlined,
  LogoutOutlined,
  FullscreenOutlined,
  FullscreenExitOutlined,
  FontSizeOutlined,
  AppstoreOutlined,
} from "@ant-design/icons";

// 默认值：优先从后端配置接口加载，回退到环境变量
const DEFAULT_ERP_URL = import.meta.env.VITE_ERP_URL || "http://localhost:3000";
const DEFAULT_SSO_URL = import.meta.env.VITE_SSO_URL || "http://localhost:3001";

/** 应用切换器（erpUrl/ssoUrl 由父组件传入，来自后端配置） */
function AppSwitcher({ erpUrl, ssoUrl }) {
  const [open, setOpen] = useState(false);
  const ref = useRef(null);

  useEffect(() => {
    const handler = (e) => {
      if (ref.current && !ref.current.contains(e.target)) setOpen(false);
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, []);

  const apps = [
    { key: "erp", label: "ERP", subtitle: "企业资源计划", href: erpUrl || DEFAULT_ERP_URL, active: false },
    { key: "mes", label: "MES", subtitle: "制造执行系统", href: null, active: true },
    { key: "sso", label: "SSO", subtitle: "认证中心", href: ssoUrl || DEFAULT_SSO_URL, active: false },
  ];

  return (
    <div ref={ref} style={{ position: "relative", display: "inline-block" }}>
      <Tooltip title="切换应用" placement="bottom">
        <Button
          type="text"
          icon={<AppstoreOutlined />}
          onClick={() => setOpen((v) => !v)}
          style={{ color: open ? "#1677ff" : "#fff" }}
        />
      </Tooltip>

      {open && (
        <div
          style={{
            position: "absolute",
            right: 0,
            top: "calc(100% + 8px)",
            zIndex: 1000,
            background: "#fff",
            border: "1px solid #e8e8e8",
            borderRadius: 4,
            padding: 8,
            minWidth: 210,
            boxShadow: "0 6px 20px rgba(0,0,0,0.14)",
          }}
        >
          <div style={{ fontSize: 11, color: "#aaa", padding: "0 8px 6px", textTransform: "uppercase", letterSpacing: 1 }}>
            AXIARZ 应用
          </div>
          {apps.map((app) => (
            <div
              key={app.key}
              onClick={() => {
                if (!app.active && app.href) {
                  window.open(app.href, "_blank", "noopener");
                  setOpen(false);
                }
              }}
              style={{
                display: "flex",
                alignItems: "center",
                gap: 10,
                padding: "8px",
                borderRadius: 4,
                cursor: app.active ? "default" : "pointer",
                opacity: app.active ? 0.7 : 1,
                background: "transparent",
              }}
              onMouseEnter={(e) => {
                if (!app.active) e.currentTarget.style.background = "#f5f5f5";
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.background = "transparent";
              }}
            >
              <div
                style={{
                  width: 32,
                  height: 32,
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  fontWeight: 700,
                  fontSize: 12,
                  borderRadius: 4,
                  background: app.active ? "#1677ff" : "#f0f0f0",
                  color: app.active ? "#fff" : "#666",
                  border: app.active ? "none" : "1px solid #e0e0e0",
                  flexShrink: 0,
                }}
              >
                {app.label}
              </div>
              <div>
                <div style={{ fontWeight: 500, fontSize: 13, color: "#333" }}>
                  {app.label}
                  {app.active && (
                    <span style={{ marginLeft: 6, fontSize: 11, color: "#1677ff" }}>当前</span>
                  )}
                </div>
                <div style={{ fontSize: 11, color: "#999" }}>{app.subtitle}</div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
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

// 根据路径获取选中的菜单项key和展开的父级菜单
function getSelectedKey(pathname) {
  const pathMap = {
    '/equipment/list': 'equipment-list',
    '/equipment/status-log': 'equipment-status-log',
    '/equipment/control': 'equipment-control',
    '/equipment/maintenance-plans': 'equipment-maintenance-plans',
    '/equipment/maintenance-tasks': 'equipment-maintenance-tasks',
    '/equipment/fault-reports': 'equipment-fault-reports',
    '/equipment/repair-orders': 'equipment-repair-orders',
    '/equipment/inspections': 'equipment-inspections',
    '/equipment/kpi': 'equipment-kpi',
    '/master/material-categories': 'master-categories',
    '/master/materials': 'master-materials',
    '/master/suppliers': 'master-suppliers',
    '/master/customers': 'master-customers',
    '/master/workshops': 'master-workshops',
    '/master/warehouses': 'master-warehouses',
    '/master/locations': 'master-locations',
    '/master/boms': 'master-boms',
    '/master/routes': 'master-routes',
    '/production/plans': 'production-plans',
    '/production/orders': 'production-orders',
    '/production/work-orders': 'production-work-orders',
    '/production/reports': 'production-reports',
    '/production/schedule': 'production-schedule',
    '/production/material-requirements': 'production-material-requirements',
    '/production/picking-orders': 'production-picking-orders',
    '/production/return-orders': 'production-return-orders',
    '/production/receipts': 'production-receipts',
    '/warehouse/inventory': 'warehouse-inventory',
    '/warehouse/inbound-orders': 'warehouse-inbound-orders',
    '/warehouse/outbound-orders': 'warehouse-outbound-orders',
    '/warehouse/transfer-orders': 'warehouse-transfer-orders',
    '/warehouse/stock-count-orders': 'warehouse-stock-count-orders',
    '/quality/inspection-tasks': 'quality-inspection-tasks',
    '/quality/inspection-reports': 'quality-inspection-reports',
    '/quality/ncr': 'quality-ncr',
    '/quality/complaints': 'quality-complaints',
    '/quality/rework-orders': 'quality-rework-orders',
    '/quality/measuring-equipment': 'quality-measuring-equipment',
    '/quality/supplier-evaluations': 'quality-supplier-evaluations',
    '/quality/traceability': 'quality-traceability',
    '/quality/costs': 'quality-costs',
    '/quality/kpi': 'quality-kpi',
    '/system/user': 'system-user',
    '/system/login-logs': 'system-login-logs',
    '/system/operation-logs': 'system-operation-logs',
    '/system/app-links': 'system-app-links'
  };

  return pathMap[pathname] || '';
}

// 根据选中的菜单项获取应该展开的父级菜单
function getOpenKeys(selectedKey) {
  const parentMap = {
    'equipment-list': 'equipment',
    'equipment-status-log': 'equipment',
    'equipment-control': 'equipment',
    'equipment-maintenance-plans': 'equipment',
    'equipment-maintenance-tasks': 'equipment',
    'equipment-fault-reports': 'equipment',
    'equipment-repair-orders': 'equipment',
    'equipment-inspections': 'equipment',
    'equipment-kpi': 'equipment',
    'master-categories': 'master',
    'master-materials': 'master',
    'master-suppliers': 'master',
    'master-customers': 'master',
    'master-workshops': 'master',
    'master-warehouses': 'master',
    'master-locations': 'master',
    'master-boms': 'master',
    'master-routes': 'master',
    'production-plans': 'production',
    'production-orders': 'production',
    'production-work-orders': 'production',
    'production-reports': 'production',
    'production-schedule': 'production',
    'production-material-requirements': 'production',
    'production-picking-orders': 'production',
    'production-return-orders': 'production',
    'production-receipts': 'production',
    'warehouse-inventory': 'warehouse',
    'warehouse-inbound-orders': 'warehouse',
    'warehouse-outbound-orders': 'warehouse',
    'warehouse-transfer-orders': 'warehouse',
    'warehouse-stock-count-orders': 'warehouse',
    'quality-inspection-tasks': 'quality',
    'quality-inspection-reports': 'quality',
    'quality-ncr': 'quality',
    'quality-complaints': 'quality',
    'quality-rework-orders': 'quality',
    'quality-measuring-equipment': 'quality',
    'quality-supplier-evaluations': 'quality',
    'quality-traceability': 'quality',
    'quality-costs': 'quality',
    'quality-kpi': 'quality',
    'system-user': 'system',
    'system-login-logs': 'system',
    'system-operation-logs': 'system',
    'system-app-links': 'system'
  };

  const parent = parentMap[selectedKey];
  return parent ? [parent] : [];
}

export default function AppRouter() {
  const location = useLocation();
  const [selectedKey, setSelectedKey] = useState('');
  const [openKeys, setOpenKeys] = useState([]);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [fontSize, setFontSize] = useState('medium');
  const [userInfo, setUserInfo] = useState(null);
  // 关联系统地址（从后端配置接口加载，供 AppSwitcher 使用）
  const [appLinks, setAppLinks] = useState({ erp_url: DEFAULT_ERP_URL, sso_url: DEFAULT_SSO_URL });

  // 获取用户信息
  useEffect(() => {
    const token = localStorage.getItem('token');
    const user = localStorage.getItem('user');
    console.log('Loading user info from localStorage:', { token: !!token, user });

    if (token && user) {
      try {
        const parsedUser = JSON.parse(user);
        console.log('Parsed user info:', parsedUser);
        setUserInfo(parsedUser);
      } catch (error) {
        console.error('Failed to parse user info:', error);
      }
    } else {
      console.log('No user info found in localStorage');
    }
  }, []);

  // 拉取关联系统地址配置
  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) return;
    fetchAppLinks()
      .then((data) => {
        if (data) setAppLinks(data);
      })
      .catch(() => { /* 保持默认值 */ });
  }, []);

  // 监听全屏状态变化
  useEffect(() => {
    const handleFullscreenChange = () => {
      setIsFullscreen(document.fullscreenElement !== null);
    };

    document.addEventListener('fullscreenchange', handleFullscreenChange);
    return () => document.removeEventListener('fullscreenchange', handleFullscreenChange);
  }, []);

  // 应用字体大小
  useEffect(() => {
    const fontSizeMap = {
      small: '12px',
      medium: '14px',
      large: '16px',
      xlarge: '18px'
    };
    document.documentElement.style.fontSize = fontSizeMap[fontSize] || '14px';
  }, [fontSize]);

  useEffect(() => {
    const key = getSelectedKey(location.pathname);
    setSelectedKey(key);

    // 平滑地更新展开状态 - 确保选中项的父级菜单被展开
    const newOpenKeys = getOpenKeys(key);
    setOpenKeys(prevOpenKeys => {
      // 如果新的父级菜单不在展开列表中，添加它
      if (newOpenKeys.length > 0 && !prevOpenKeys.includes(newOpenKeys[0])) {
        return [...prevOpenKeys, ...newOpenKeys];
      }
      // 如果已经在展开列表中或没有父级菜单，保持不变
      return prevOpenKeys;
    });
  }, [location.pathname]);

  const handleMenuClick = (item) => {
    setSelectedKey(item.key);

    // 平滑地更新展开状态 - 确保选中项的父级菜单被展开
    const newOpenKeys = getOpenKeys(item.key);
    setOpenKeys(prevOpenKeys => {
      // 如果新的父级菜单不在展开列表中，添加它
      if (newOpenKeys.length > 0 && !prevOpenKeys.includes(newOpenKeys[0])) {
        return [...prevOpenKeys, ...newOpenKeys];
      }
      // 如果已经在展开列表中，保持不变
      return prevOpenKeys;
    });

    // 执行原有的跳转逻辑
    const pathMap = {
      'equipment-list': '/equipment/list',
      'equipment-status-log': '/equipment/status-log',
      'equipment-control': '/equipment/control',
      'equipment-maintenance-plans': '/equipment/maintenance-plans',
      'equipment-maintenance-tasks': '/equipment/maintenance-tasks',
      'equipment-fault-reports': '/equipment/fault-reports',
      'equipment-repair-orders': '/equipment/repair-orders',
      'equipment-inspections': '/equipment/inspections',
      'equipment-kpi': '/equipment/kpi',
      'master-categories': '/master/material-categories',
      'master-materials': '/master/materials',
      'master-suppliers': '/master/suppliers',
      'master-customers': '/master/customers',
      'master-workshops': '/master/workshops',
      'master-warehouses': '/master/warehouses',
      'master-locations': '/master/locations',
      'master-boms': '/master/boms',
      'master-routes': '/master/routes',
      'production-plans': '/production/plans',
      'production-orders': '/production/orders',
      'production-work-orders': '/production/work-orders',
      'production-reports': '/production/reports',
      'production-schedule': '/production/schedule',
      'production-material-requirements': '/production/material-requirements',
      'production-picking-orders': '/production/picking-orders',
      'production-return-orders': '/production/return-orders',
      'production-receipts': '/production/receipts',
      'warehouse-inventory': '/warehouse/inventory',
      'warehouse-inbound-orders': '/warehouse/inbound-orders',
      'warehouse-outbound-orders': '/warehouse/outbound-orders',
      'warehouse-transfer-orders': '/warehouse/transfer-orders',
      'warehouse-stock-count-orders': '/warehouse/stock-count-orders',
      'quality-inspection-tasks': '/quality/inspection-tasks',
      'quality-inspection-reports': '/quality/inspection-reports',
      'quality-ncr': '/quality/ncr',
      'quality-complaints': '/quality/complaints',
      'quality-rework-orders': '/quality/rework-orders',
      'quality-measuring-equipment': '/quality/measuring-equipment',
      'quality-supplier-evaluations': '/quality/supplier-evaluations',
      'quality-traceability': '/quality/traceability',
      'quality-costs': '/quality/costs',
      'quality-kpi': '/quality/kpi',
      'system-user': '/system/user',
      'system-login-logs': '/system/login-logs',
      'system-operation-logs': '/system/operation-logs',
      'system-app-links': '/system/app-links'
    };

    const path = pathMap[item.key];
    if (path) {
      window.location.href = path;
    }
  };

  // 全屏切换
  const toggleFullscreen = () => {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen().catch(err => {
        console.error('Failed to enter fullscreen:', err);
      });
    } else {
      document.exitFullscreen().catch(err => {
        console.error('Failed to exit fullscreen:', err);
      });
    }
  };

  // 退出登录
  const handleLogout = () => {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
    setUserInfo(null);
    message.success('已退出登录');
    window.location.href = '/login';
  };

  // 用户菜单项
  const userMenuItems = [
    {
      key: 'profile',
      label: '个人资料',
      icon: <UserOutlined />,
      disabled: true, // 暂时禁用
    },
    {
      type: 'divider',
    },
    {
      key: 'logout',
      label: '退出登录',
      icon: <LogoutOutlined />,
      onClick: handleLogout,
    },
  ];

  return (
    <Layout style={{ minHeight: "100vh" }}>
      <Header style={{
        color: "#fff",
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
        padding: "0 20px"
      }}>
        <div style={{ fontSize: 18 }}>MES 系统</div>
        <Space>
          {/* 应用切换器（URL 来自后端配置，可在系统设置 → 关联系统地址 中修改） */}
          <AppSwitcher erpUrl={appLinks.erp_url} ssoUrl={appLinks.sso_url} />

          {/* 字体大小控制 */}
          <Select
            value={fontSize}
            onChange={setFontSize}
            style={{ width: 100 }}
            suffixIcon={<FontSizeOutlined />}
            options={[
              { value: 'small', label: '小' },
              { value: 'medium', label: '中' },
              { value: 'large', label: '大' },
              { value: 'xlarge', label: '特大' },
            ]}
          />

          {/* 全屏按钮 */}
          <Button
            type="text"
            icon={isFullscreen ? <FullscreenExitOutlined /> : <FullscreenOutlined />}
            onClick={toggleFullscreen}
            style={{ color: "#fff" }}
          />

          {/* 用户菜单 */}
          {userInfo && (
            <Dropdown
              menu={{ items: userMenuItems }}
              placement="bottomRight"
              arrow
            >
              <Button type="text" style={{ color: "#fff", padding: "0 8px" }}>
                <Space>
                  <Avatar size="small" icon={<UserOutlined />} />
                  {userInfo.real_name || userInfo.username}
                </Space>
              </Button>
            </Dropdown>
          )}
        </Space>
      </Header>
      <Layout>
        <Sider width={220} theme="light">
          <Menu
            mode="inline"
            selectedKeys={[selectedKey]}
            openKeys={openKeys}
            onClick={handleMenuClick}
            onOpenChange={(keys) => setOpenKeys(keys)}
            items={[
              {
                key: "equipment",
                label: "设备管理",
                children: [
                  { key: "equipment-list", label: "设备台账" },
                  { key: "equipment-status-log", label: "状态日志" },
                  { key: "equipment-control", label: "设备控制" },
                  { key: "equipment-maintenance-plans", label: "维护计划" },
                  { key: "equipment-maintenance-tasks", label: "维护任务" },
                  { key: "equipment-fault-reports", label: "故障报修" },
                  { key: "equipment-repair-orders", label: "维修工单" },
                  { key: "equipment-inspections", label: "点检记录" },
                  { key: "equipment-kpi", label: "设备KPI" }
                ]
              },
              {
                key: "master",
                label: "主数据管理",
                children: [
                  { key: "master-categories", label: "物料分类" },
                  { key: "master-materials", label: "物料主数据" },
                  { key: "master-suppliers", label: "供应商" },
                  { key: "master-customers", label: "客户" },
                  { key: "master-workshops", label: "车间" },
                  { key: "master-warehouses", label: "仓库" },
                  { key: "master-locations", label: "库位" },
                  { key: "master-boms", label: "BOM 管理" },
                  { key: "master-routes", label: "工艺路线" }
                ]
              },
              {
                key: "production",
                label: "生产管理",
                children: [
                  { key: "production-plans", label: "生产计划" },
                  { key: "production-orders", label: "生产订单" },
                  { key: "production-work-orders", label: "生产工单" },
                  { key: "production-reports", label: "生产报工" },
                  { key: "production-schedule", label: "生产排程" },
                  { key: "production-material-requirements", label: "物料需求" },
                  { key: "production-picking-orders", label: "领料单" },
                  { key: "production-return-orders", label: "退料单" },
                  { key: "production-receipts", label: "完工入库" }
                ]
              },
              {
                key: "warehouse",
                label: "仓储管理",
                children: [
                  { key: "warehouse-inventory", label: "库存总览" },
                  { key: "warehouse-inbound-orders", label: "入库单" },
                  { key: "warehouse-outbound-orders", label: "出库单" },
                  { key: "warehouse-transfer-orders", label: "调拨单" },
                  { key: "warehouse-stock-count-orders", label: "盘点单" }
                ]
              },
              {
                key: "quality",
                label: "质量管理",
                children: [
                  { key: "quality-inspection-tasks", label: "质检任务" },
                  { key: "quality-inspection-reports", label: "质检报告" },
                  { key: "quality-ncr", label: "不合格品（NCR）" },
                  { key: "quality-complaints", label: "客户投诉" },
                  { key: "quality-rework-orders", label: "返工单" },
                  { key: "quality-measuring-equipment", label: "测量设备" },
                  { key: "quality-supplier-evaluations", label: "供应商评估" },
                  { key: "quality-traceability", label: "质量追溯" },
                  { key: "quality-costs", label: "质量成本" },
                  { key: "quality-kpi", label: "质量KPI看板" }
                ]
              },
              {
                key: "system",
                label: "系统管理",
                children: [
                  { key: "system-user", label: "用户管理（SSO）" },
                  { key: "system-login-logs", label: "登录日志" },
                  { key: "system-operation-logs", label: "操作审计" },
                  { key: "system-app-links", label: "关联系统地址" }
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


