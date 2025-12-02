import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message, InputNumber } from "antd";
import {
  fetchWorkOrders,
  createWorkOrder,
  updateWorkOrder,
  deleteWorkOrder
} from "../../api/productionApi";

const statusMap = {
  1: { text: "待开工", color: "default" },
  2: { text: "生产中", color: "gold" },
  3: { text: "暂停", color: "orange" },
  4: { text: "完工", color: "green" },
  5: { text: "取消", color: "red" }
};

export default function WorkOrderList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchWorkOrders({ page: 0, page_size: 50 });
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
        work_order_status: 1
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    if (current) {
      await updateWorkOrder(current.id, values);
      message.success("更新成功");
    } else {
      await createWorkOrder(values);
      message.success("创建成功");
    }
    setModalOpen(false);
    load();
  };

  const handleDelete = async (record) => {
    await deleteWorkOrder(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "工单号", dataIndex: "work_order_no" },
    { title: "生产订单", dataIndex: "production_order_id" },
    { title: "工序ID", dataIndex: "process_id" },
    {
      title: "计划数量",
      dataIndex: "plan_quantity",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    {
      title: "状态",
      dataIndex: "work_order_status",
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
      title="生产工单"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建工单
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
      <Modal
        open={modalOpen}
        title={current ? "编辑工单" : "新建工单"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="work_order_no" label="工单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="production_order_id" label="生产订单ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="process_id" label="工序ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="plan_quantity" label="计划数量" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} min={0} />
          </Form.Item>
          <Form.Item name="work_order_status" label="状态" rules={[{ required: true }]}>
            <Select
              options={Object.entries(statusMap).map(([value, item]) => ({
                value: Number(value),
                label: item.text
              }))}
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


