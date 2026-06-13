import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, DatePicker, Modal, Select, Space, Table, Tag, message } from "antd";
import dayjs from "dayjs";
import {
  fetchProductionPlans,
  createProductionPlan,
  updateProductionPlan,
  deleteProductionPlan
} from "../../api/productionApi";

const statusMap = {
  1: { text: "未开始", color: "default" },
  2: { text: "执行中", color: "cyan" },
  3: { text: "已完成", color: "green" },
  4: { text: "已取消", color: "red" }
};

export default function ProductionPlanList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchProductionPlans({ page: 0, page_size: 50 });
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
        ...record,
        plan_start_date: dayjs(record.plan_start_date),
        plan_end_date: dayjs(record.plan_end_date)
      });
    } else {
      form.setFieldsValue({
        plan_status: 1,
        plan_type: 1,
        plan_start_date: dayjs(),
        plan_end_date: dayjs().add(7, "day")
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      plan_start_date: values.plan_start_date.format("YYYY-MM-DD"),
      plan_end_date: values.plan_end_date.format("YYYY-MM-DD")
    };
    if (current) {
      await updateProductionPlan(current.id, payload);
      message.success("更新成功");
    } else {
      await createProductionPlan(payload);
      message.success("创建成功");
    }
    setModalOpen(false);
    load();
  };

  const handleDelete = async (record) => {
    await deleteProductionPlan(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "计划编号", dataIndex: "plan_no" },
    { title: "计划名称", dataIndex: "plan_name" },
    { title: "类型", dataIndex: "plan_type" },
    {
      title: "状态",
      dataIndex: "plan_status",
      render: (v) => <Tag color={statusMap[v]?.color}>{statusMap[v]?.text || "未知"}</Tag>
    },
    { title: "开始日期", dataIndex: "plan_start_date" },
    { title: "结束日期", dataIndex: "plan_end_date" },
    {
      title: "完成率",
      dataIndex: "completion_rate",
      render: (v) => `${(v * 100).toFixed(1)}%`
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
      title="生产计划"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建计划
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
      <Modal
        open={modalOpen}
        title={current ? "编辑生产计划" : "新建生产计划"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="plan_no" label="计划编号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="plan_name" label="计划名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="plan_type" label="计划类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "月计划" },
                { value: 2, label: "周计划" },
                { value: 3, label: "日计划" },
                { value: 4, label: "临时计划" }
              ]}
            />
          </Form.Item>
          <Form.Item name="plan_status" label="状态" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "未开始" },
                { value: 2, label: "执行中" },
                { value: 3, label: "已完成" },
                { value: 4, label: "已取消" }
              ]}
            />
          </Form.Item>
          <Form.Item name="plan_period" label="计划周期">
            <Input placeholder="2025-01" />
          </Form.Item>
          <Form.Item name="plan_start_date" label="开始日期" rules={[{ required: true }]}>
            <DatePicker />
          </Form.Item>
          <Form.Item name="plan_end_date" label="结束日期" rules={[{ required: true }]}>
            <DatePicker />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


