import React from "react";
import { Button, Result } from "antd";
import { TeamOutlined } from "@ant-design/icons";

const SSO_URL = import.meta.env.VITE_SSO_URL || "http://localhost:3001";

/**
 * 用户管理已迁移至 SSO 统一管理。
 * 此页面仅作为跳转入口，不再在 MES 本地管理用户。
 */
export default function UserList() {
  return (
    <div style={{ padding: 40 }}>
      <Result
        icon={<TeamOutlined style={{ color: "#1677ff" }} />}
        title="用户管理已由 SSO 统一管理"
        subTitle="用户账号、角色与权限现在统一在「认证中心（SSO）」中管理。点击下方按钮前往 SSO 管理控制台进行用户操作。"
        extra={[
          <Button
            key="sso"
            type="primary"
            href={`${SSO_URL}/users`}
            target="_blank"
            rel="noopener noreferrer"
          >
            前往 SSO 用户管理
          </Button>,
          <Button
            key="roles"
            href={`${SSO_URL}/roles`}
            target="_blank"
            rel="noopener noreferrer"
          >
            前往 SSO 角色管理
          </Button>,
        ]}
      />
    </div>
  );
}
