import React, { useEffect } from "react";
import { Button, Spin } from "antd";
import { redirectToSsoLogin } from "../../api/authApi";

/**
 * MES 登录页（方案 A：认证完全交由 SSO）
 * 支持账号密码登录和飞书扫码，均在 SSO 侧完成。
 */
export default function Login() {
  const token = localStorage.getItem("token");

  useEffect(() => {
    if (token) {
      window.location.replace("/");
    }
  }, [token]);

  if (token) {
    return (
      <div style={{ display: "flex", justifyContent: "center", alignItems: "center", height: "100vh" }}>
        <Spin tip="正在跳转..." size="large" />
      </div>
    );
  }

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
        height: "100vh",
        background: "#f0f2f5",
        gap: 0,
      }}
    >
      <div
        style={{
          background: "#fff",
          borderRadius: 8,
          padding: "48px 40px 36px",
          width: 380,
          boxShadow: "0 4px 24px rgba(0,0,0,0.08)",
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          gap: 24,
        }}
      >
        {/* Logo 区 */}
        <div style={{ textAlign: "center" }}>
          <div
            style={{
              width: 56,
              height: 56,
              background: "#1677ff",
              borderRadius: 12,
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              margin: "0 auto 12px",
            }}
          >
            <span style={{ color: "#fff", fontSize: 24, fontWeight: 700 }}>M</span>
          </div>
          <h2 style={{ margin: 0, fontSize: 22, fontWeight: 700, color: "#1a1a1a" }}>
            MES 制造执行系统
          </h2>
          <p style={{ margin: "6px 0 0", color: "#8c8c8c", fontSize: 13 }}>
            请通过统一认证中心登录
          </p>
        </div>

        {/* 登录说明 */}
        <div
          style={{
            background: "#f6f8ff",
            border: "1px solid #d6e4ff",
            borderRadius: 6,
            padding: "10px 14px",
            width: "100%",
            fontSize: 13,
            color: "#595959",
            lineHeight: 1.6,
          }}
        >
          支持 <strong>账号密码</strong> 及{" "}
          <strong style={{ color: "#1456F0" }}>飞书扫码</strong>{" "}
          两种方式，均在统一认证中心完成。
        </div>

        {/* 主按钮 */}
        <Button
          type="primary"
          size="large"
          block
          onClick={redirectToSsoLogin}
          style={{ height: 44, fontSize: 15, letterSpacing: "0.05em" }}
        >
          前往统一认证中心登录
        </Button>

        <p style={{ fontSize: 12, color: "#bfbfbf", margin: 0 }}>
          © 2026 AXIARZ MES Platform
        </p>
      </div>
    </div>
  );
}



