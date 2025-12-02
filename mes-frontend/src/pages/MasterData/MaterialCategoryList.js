import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message } from "antd";
import {
  fetchMaterialCategories,
  createMaterialCategory,
  updateMaterialCategory,
  deleteMaterialCategory
} from "../../api/masterDataApi";

const statusMap = {
  0: { text: "禁用", color: "red" },
  1: { text: "启用", color: "green" }
};

export default function MaterialCategoryList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchMaterialCategories({ page: 0, page_size: 100 });
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
      form.setFieldsValue({ status: 1, parent_id: 0, sort_order: 0 });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    if (current) {
      await updateMaterialCategory(current.id, values);
      message.success("更新成功");
    } else {
      await createMaterialCategory(values);
      message.success("创建成功");
    }
    setModalOpen(false);
    load();
  };

  const handleDelete = async (record) => {
    await deleteMaterialCategory(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "编码", dataIndex: "category_code" },
    { title: "名称", dataIndex: "category_name" },
    { title: "父级ID", dataIndex: "parent_id" },
    { title: "层级", dataIndex: "category_level" },
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
      title="物料分类"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新增分类
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
      <Modal
        open={modalOpen}
        title={current ? "编辑分类" : "新增分类"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="category_code" label="分类编码" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="category_name" label="分类名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="parent_id" label="父级ID">
            <Input type="number" />
          </Form.Item>
          <Form.Item name="category_level" label="层级">
            <Input type="number" />
          </Form.Item>
          <Form.Item name="sort_order" label="排序">
            <Input type="number" />
          </Form.Item>
          <Form.Item name="status" label="状态" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "启用" },
                { value: 0, label: "禁用" }
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


