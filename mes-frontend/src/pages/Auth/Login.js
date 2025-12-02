import React from "react";
import { Card, Form, Input, Button, message } from "antd";
import { login } from "../../api/authApi";

export default function Login() {
  const [form] = Form.useForm();

  const onFinish = async (values) => {
    try {
      const res = await login(values);
      localStorage.setItem("token", res.token);
      message.success("登录成功");
      window.location.href = "/";
    } catch (e) {
      message.error("登录失败");
    }
  };

  return (
    <div style={{ display: "flex", justifyContent: "center", alignItems: "center", height: "100vh" }}>
      <Card title="MES 登录" style={{ width: 360 }}>
        <Form form={form} layout="vertical" onFinish={onFinish}>
          <Form.Item name="username" label="用户名" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="password" label="密码" rules={[{ required: true }]}>
            <Input.Password />
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit" block>
              登录
            </Button>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
}


