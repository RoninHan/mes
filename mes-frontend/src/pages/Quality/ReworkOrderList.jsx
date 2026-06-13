import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message } from "antd";
import { fetchReworkOrders, createReworkOrder, updateReworkOrder, deleteReworkOrder } from "../../api/qualityApi";

const reworkStatusMap = {
  1: { text: "待返工", color: "default" },
  2: { text: "返工中", color: "processing" },
  3: { text: "已完成", color: "success" },
  4: { text: "已取消", color: "error" }
};

export default function ReworkOrderList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchReworkOrders({ page: 0, page_size: 50 });
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
        rework_type: 1,
        rework_status: 1
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      rework_quantity: parseFloat(values.rework_quantity) || 0
    };
    try {
      if (current) {
        await updateReworkOrder(current.id, payload);
        message.success("更新成功");
      } else {
        await createReworkOrder(payload);
        message.success("创建成功");
      }
      setModalOpen(false);
      load();
    } catch (error) {
      message.error(error.response?.data?.message || "操作失败");
    }
  };

  const handleDelete = async (record) => {
    try {
      await deleteReworkOrder(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "返工单号", dataIndex: "rework_no", width: 150 },
    { title: "NCR ID", dataIndex: "ncr_id", width: 100 },
    { title: "物料ID", dataIndex: "material_id", width: 100 },
    { title: "批次号", dataIndex: "batch_no", width: 120 },
    { title: "返工数量", dataIndex: "rework_quantity", width: 100 },
    { title: "已完成数量", dataIndex: "completed_quantity", width: 120 },
    { title: "合格数量", dataIndex: "qualified_quantity", width: 100 },
    {
      title: "返工状态",
      dataIndex: "rework_status",
      width: 100,
      render: (v) => <Tag color={reworkStatusMap[v]?.color}>{reworkStatusMap[v]?.text || "未知"}</Tag>
    },
    {
      title: "操作",
      width: 150,
      fixed: "right",
      render: (_, record) => (
        <Space>
          <Button type="link" size="small" onClick={() => openModal(record)}>
            编辑
          </Button>
          <Button type="link" size="small" danger onClick={() => handleDelete(record)}>
            删除
          </Button>
        </Space>
      )
    }
  ];

  return (
    <Card
      title="返工单管理"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建返工单
        </Button>
      }
    >
      <Table
        columns={columns}
        dataSource={data}
        loading={loading}
        rowKey="id"
        scroll={{ x: 1000 }}
        pagination={{ pageSize: 50 }}
      />

      <Modal
        title={current ? "编辑返工单" : "新建返工单"}
        open={modalOpen}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="rework_no" label="返工单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="ncr_id" label="NCR ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="material_id" label="物料ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="rework_quantity" label="返工数量" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="unit" label="单位" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="rework_type" label="返工类型" rules={[{ required: true }]}>
            <Select>
              <Select.Option value={1}>工序返工</Select.Option>
              <Select.Option value={2}>全检挑选</Select.Option>
              <Select.Option value={3}>返修</Select.Option>
              <Select.Option value={4}>其他</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="rework_reason" label="返工原因" rules={[{ required: true }]}>
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


