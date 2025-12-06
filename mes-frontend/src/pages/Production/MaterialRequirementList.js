import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, InputNumber, Modal, Space, Table, message } from "antd";
import {
  fetchMaterialRequirements,
  createMaterialRequirement,
  updateMaterialRequirement,
  deleteMaterialRequirement
} from "../../api/productionApi";

export default function MaterialRequirementList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchMaterialRequirements({ page: 0, page_size: 50 });
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
      form.setFieldsValue({
        required_quantity: 0,
        reserved_quantity: 0,
        issued_quantity: 0
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    try {
      if (current) {
        await updateMaterialRequirement(current.id, values);
        message.success("更新成功");
      } else {
        await createMaterialRequirement(values);
        message.success("创建成功");
      }
      setModalOpen(false);
      load();
    } catch (e) {
      message.error("保存失败");
    }
  };

  const handleDelete = async (record) => {
    await deleteMaterialRequirement(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "生产订单ID", dataIndex: "production_order_id" },
    { title: "物料ID", dataIndex: "material_id" },
    {
      title: "需求数量",
      dataIndex: "required_quantity",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    {
      title: "已预留",
      dataIndex: "reserved_quantity",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    {
      title: "已发料",
      dataIndex: "issued_quantity",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    { title: "单位", dataIndex: "unit" },
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
      title="生产物料需求"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建物料需求
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />

      <Modal
        open={modalOpen}
        title={current ? "编辑物料需求" : "新建物料需求"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="production_order_id"
            label="生产订单ID"
            rules={[{ required: true }]}
          >
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="material_id" label="物料ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item
            name="required_quantity"
            label="需求数量"
            rules={[{ required: true }]}
          >
            <InputNumber min={0} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="reserved_quantity" label="已预留数量">
            <InputNumber min={0} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="issued_quantity" label="已发料数量">
            <InputNumber min={0} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="unit" label="单位" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


