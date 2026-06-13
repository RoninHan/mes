import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, message, Spin } from "antd";
import { LinkOutlined } from "@ant-design/icons";
import { fetchAppLinks, updateAppLinks } from "../../api/systemApi";

/**
 * 关联系统地址配置页
 * 配置 ERP / SSO 的前端访问地址，供 Header 应用切换器使用。
 * 保存后立即生效（热更新内存，无需重启服务）。
 */
export default function AppLinksSettings({ onLinksChange }) {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  const load = async () => {
    setLoading(true);
    try {
      const data = await fetchAppLinks();
      form.setFieldsValue(data);
    } catch (e) {
      message.error("加载配置失败: " + (e?.message || "未知错误"));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
  }, []);

  const handleSave = async () => {
    const values = await form.validateFields();
    setSaving(true);
    try {
      const updated = await updateAppLinks(values);
      form.setFieldsValue(updated);
      // 通知父组件（AppRouter）更新应用切换器的 URL
      if (onLinksChange) onLinksChange(updated);
      message.success("关联系统地址已保存，应用切换器立即生效");
    } catch (e) {
      message.error("保存失败: " + (e?.message || "未知错误"));
    } finally {
      setSaving(false);
    }
  };

  return (
    <div style={{ padding: 24, maxWidth: 640 }}>
      <div style={{ marginBottom: 16 }}>
        <h2 style={{ fontSize: 16, fontWeight: 600 }}>关联系统地址</h2>
        <p style={{ color: "#888", fontSize: 13, marginTop: 4 }}>
          配置 ERP、SSO 的前端访问地址。Header 顶部的应用切换器将使用这里设置的地址，
          无需修改环境变量或重启服务。
        </p>
      </div>

      <Card loading={loading}>
        <Form form={form} layout="vertical">
          <Form.Item
            name="erp_url"
            label="ERP 系统地址"
            extra="ERP 前端的访问地址，如 http://erp.company.com 或 http://localhost:3000"
          >
            <Input
              prefix={<LinkOutlined />}
              placeholder="http://localhost:3000"
            />
          </Form.Item>

          <Form.Item
            name="sso_url"
            label="SSO 认证中心地址"
            extra="SSO 前端的访问地址，如 http://sso.company.com 或 http://localhost:3001"
          >
            <Input
              prefix={<LinkOutlined />}
              placeholder="http://localhost:3001"
            />
          </Form.Item>

          <Form.Item style={{ marginBottom: 0 }}>
            <Button type="primary" loading={saving} onClick={handleSave}>
              保存并立即生效
            </Button>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
}
