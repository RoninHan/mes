import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message } from "antd";
import { fetchUsers, createUser, updateUser, deleteUser } from "../../api/systemApi";

const statusMap = {
  0: { text: "禁用", color: "red" },
  1: { text: "启用", color: "green" }
};

export default function UserList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchUsers({ page: 0, page_size: 50 });
      setData(res.items || []);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
  }, []);

  const openModal = (record) => {
    setCurrent(record || null);
    setModalOpen(true);
    form.resetFields();
    if (record) {
      form.setFieldsValue({
        username: record.username,
        real_name: record.real_name,
        dept_id: record.dept_id,
        email: record.email,
        phone: record.phone,
        status: record.status
      });
    } else {
      form.setFieldsValue({ status: 1 });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      password: values.password || "ChangeMe123!"
    };
    if (current) {
      await updateUser(current.id, payload);
      message.success("更新成功");
    } else {
      await createUser(payload);
      message.success("创建成功");
    }
    setModalOpen(false);
    load();
  };

  const handleDelete = async (record) => {
    await deleteUser(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "用户名", dataIndex: "username" },
    { title: "姓名", dataIndex: "real_name" },
    { title: "部门ID", dataIndex: "dept_id" },
    { title: "邮箱", dataIndex: "email" },
    { title: "手机号", dataIndex: "phone" },
    {
      title: "状态",
      dataIndex: "status",
      render: (v) => <Tag color={statusMap[v]?.color}>{statusMap[v]?.text || "未知"}</Tag>
    },
    {
      title: "操作",
      render: (_, record) => (
        <Space>
          <Button type="link" onClick={() => openModal(record)}>
            编辑
          </Button>
          <Button type="link" danger onClick={() => handleDelete(record)}>
            删除
          </Button>
        </Space>
      )
    }
  ];

  return (
    <Card
      title="用户管理"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新增用户
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
      <Modal
        open={modalOpen}
        title={current ? "编辑用户" : "新增用户"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="username" label="用户名" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="real_name" label="姓名" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="password" label="密码" extra="留空则保持不变 / 使用默认值">
            <Input.Password />
          </Form.Item>
          <Form.Item name="dept_id" label="部门ID">
            <Input />
          </Form.Item>
          <Form.Item name="email" label="邮箱">
            <Input />
          </Form.Item>
          <Form.Item name="phone" label="手机号">
            <Input />
          </Form.Item>
          <Form.Item name="status" label="状态" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "启用" },
                { value: 0, label: "禁用" }
              ]}
            />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


