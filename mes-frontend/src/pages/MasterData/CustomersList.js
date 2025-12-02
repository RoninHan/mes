import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message } from "antd";
import {
  fetchCustomers,
  createCustomer,
  updateCustomer,
  deleteCustomer
} from "../../api/masterDataApi";

const statusMap = {
  0: { text: "禁用", color: "red" },
  1: { text: "启用", color: "green" },
  2: { text: "黑名单", color: "orange" }
};

export default function CustomersList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchCustomers({ page: 0, page_size: 50 });
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
      form.setFieldsValue(record);
    } else {
      form.setFieldsValue({ status: 1, customer_type: 1 });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    if (current) {
      await updateCustomer(current.id, values);
      message.success("更新成功");
    } else {
      await createCustomer(values);
      message.success("创建成功");
    }
    setModalOpen(false);
    load();
  };

  const handleDelete = async (record) => {
    await deleteCustomer(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "编码", dataIndex: "customer_code" },
    { title: "名称", dataIndex: "customer_name" },
    { title: "类型", dataIndex: "customer_type" },
    { title: "等级", dataIndex: "customer_level" },
    { title: "联系人", dataIndex: "contact_person" },
    { title: "电话", dataIndex: "contact_phone" },
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
      title="客户"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新增客户
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
      <Modal
        open={modalOpen}
        title={current ? "编辑客户" : "新增客户"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="customer_code" label="编码" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="customer_name" label="名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="customer_type" label="类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "直接客户" },
                { value: 2, label: "经销商" },
                { value: 3, label: "代理商" },
                { value: 4, label: "终端客户" }
              ]}
            />
          </Form.Item>
          <Form.Item name="customer_level" label="等级">
            <Select
              allowClear
              options={[
                { value: "A", label: "A" },
                { value: "B", label: "B" },
                { value: "C", label: "C" }
              ]}
            />
          </Form.Item>
          <Form.Item name="contact_person" label="联系人">
            <Input />
          </Form.Item>
          <Form.Item name="contact_phone" label="联系电话">
            <Input />
          </Form.Item>
          <Form.Item name="status" label="状态" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "启用" },
                { value: 0, label: "禁用" },
                { value: 2, label: "黑名单" }
              ]}
            />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


