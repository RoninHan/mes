import React, { useEffect, useRef } from "react";
import { Spin, message } from "antd";
import { extractSsoToken } from "../../api/authApi";

/**
 * MES SSO 回调页 /auth/callback
 * SSO 登录成功后重定向至此，URL 中携带 sso_token 参数
 */
export default function SsoCallback() {
  const handled = useRef(false);

  useEffect(() => {
    if (handled.current) return;
    handled.current = true;

    const token = extractSsoToken();
    if (token) {
      message.success("登录成功");
      window.location.replace("/");
    } else {
      message.error("登录失败，未获取到有效 Token");
      window.location.replace("/login");
    }
  }, []);

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
        height: "100vh",
        gap: 16,
        background: "#f0f2f5",
      }}
    >
      <Spin size="large" />
      <p style={{ fontSize: 14, color: "#8c8c8c" }}>正在完成登录，请稍候…</p>
    </div>
  );
}
