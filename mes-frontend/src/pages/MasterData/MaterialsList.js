import React, { useEffect, useState } from "react";
import {
  Button,
  Card,
  Form,
  Input,
  Modal,
  Select,
  Space,
  Table,
  Tag,
  message
} from "antd";
import {
  fetchMaterials,
  createMaterial,
  updateMaterial,
  deleteMaterial
} from "../../api/masterDataApi";

const statusMap = {
  0: { text: "停用", color: "red" },
  1: { text: "启用", color: "green" }
};

export default function MaterialsList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchMaterials({ page: 0, page_size: 50 });
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
      form.setFieldsValue({ status: 1, material_type: 1 });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    if (current) {
      await updateMaterial(current.id, values);
      message.success("更新成功");
    } else {
      await createMaterial(values);
      message.success("创建成功");
    }
    setModalOpen(false);
    load();
  };

  const handleDelete = async (record) => {
    await deleteMaterial(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "物料编码", dataIndex: "material_code" },
    { title: "物料名称", dataIndex: "material_name" },
    { title: "规格", dataIndex: "material_spec" },
    { title: "分类ID", dataIndex: "category_id" },
    {
      title: "类型",
      dataIndex: "material_type",
      render: (v) => ["", "原材料", "半成品", "成品", "辅料", "包装"][v] || v
    },
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
      title="物料主数据"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新增物料
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
      <Modal
        open={modalOpen}
        title={current ? "编辑物料" : "新增物料"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="material_code" label="物料编码" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="material_name" label="物料名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="material_spec" label="规格">
            <Input />
          </Form.Item>
          <Form.Item name="material_model" label="型号">
            <Input />
          </Form.Item>
          <Form.Item name="category_id" label="分类ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="material_type" label="物料类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "原材料" },
                { value: 2, label: "半成品" },
                { value: 3, label: "成品" },
                { value: 4, label: "辅料" },
                { value: 5, label: "包装材料" }
              ]}
            />
          </Form.Item>
          <Form.Item name="unit" label="单位" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="status" label="状态" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "启用" },
                { value: 0, label: "停用" }
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


