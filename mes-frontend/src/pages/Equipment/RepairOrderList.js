import React, { useEffect, useState } from "react";
import { Button, Card, DatePicker, Form, Input, InputNumber, Modal, Select, Space, Table, Tag, message } from "antd";
import dayjs from "dayjs";
import {
  fetchRepairOrders,
  createRepairOrder,
  updateRepairOrder,
  deleteRepairOrder
} from "../../api/equipmentApi";

const statusMap = {
  1: { text: "待维修", color: "default" },
  2: { text: "维修中", color: "blue" },
  3: { text: "已完成", color: "green" },
  4: { text: "已关闭", color: "red" }
};

export default function RepairOrderList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchRepairOrders({ page: 0, page_size: 50 });
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
    form.resetFields();
    if (record) {
      form.setFieldsValue({
        ...record,
        start_time: record.start_time ? dayjs(record.start_time) : null,
        end_time: record.end_time ? dayjs(record.end_time) : null
      });
    } else {
      form.setFieldsValue({
        repair_type: 1,
        status: 1
      });
    }
    setModalOpen(true);
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      start_time: values.start_time?.toISOString() || null,
      end_time: values.end_time?.toISOString() || null
    };
    try {
      if (current) {
        await updateRepairOrder(current.id, payload);
        message.success("更新成功");
      } else {
        await createRepairOrder(payload);
        message.success("创建成功");
      }
      setModalOpen(false);
      load();
    } catch {
      message.error("保存失败");
    }
  };

  const handleDelete = async (record) => {
    await deleteRepairOrder(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "维修单号", dataIndex: "repair_no" },
    { title: "故障ID", dataIndex: "fault_id" },
    { title: "设备ID", dataIndex: "equipment_id" },
    { title: "维修类型", dataIndex: "repair_type" },
    { title: "开始时间", dataIndex: "start_time" },
    { title: "结束时间", dataIndex: "end_time" },
    { title: "停机时长", dataIndex: "downtime_minutes" },
    {
      title: "人工成本",
      dataIndex: "cost_labor",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    {
      title: "备件成本",
      dataIndex: "cost_spare_parts",
      render: (v) => v?.toFixed?.(2) ?? v
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
      title="设备维修工单"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建维修工单
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />
      <Modal
        open={modalOpen}
        title={current ? "编辑维修工单" : "新建维修工单"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="repair_no" label="维修单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="fault_id" label="故障ID">
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="equipment_id" label="设备ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="repair_type" label="维修类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "故障维修" },
                { value: 2, label: "预防性维修" },
                { value: 3, label: "改造" }
              ]}
            />
          </Form.Item>
          <Form.Item name="start_time" label="开始时间">
            <DatePicker showTime />
          </Form.Item>
          <Form.Item name="end_time" label="结束时间">
            <DatePicker showTime />
          </Form.Item>
          <Form.Item name="downtime_minutes" label="停机时长(分钟)">
            <InputNumber min={0} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="repair_person_id" label="维修人ID">
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="cost_labor" label="人工成本" rules={[{ required: true }]}>
            <InputNumber min={0} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="cost_spare_parts" label="备件成本" rules={[{ required: true }]}>
            <InputNumber min={0} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="status" label="状态" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "待维修" },
                { value: 2, label: "维修中" },
                { value: 3, label: "已完成" },
                { value: 4, label: "已关闭" }
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



