import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, InputNumber, Modal, Select, Space, Table, Tag, DatePicker, message } from "antd";
import dayjs from "dayjs";
import {
  fetchMaintenancePlans,
  createMaintenancePlan,
  updateMaintenancePlan,
  deleteMaintenancePlan
} from "../../api/equipmentApi";

const statusMap = {
  0: { text: "停用", color: "red" },
  1: { text: "启用", color: "green" }
};

export default function MaintenancePlanList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchMaintenancePlans({ page: 0, page_size: 50 });
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
        next_due_time: record.next_due_time ? dayjs(record.next_due_time) : null
      });
    } else {
      form.setFieldsValue({
        plan_type: 1,
        cycle_type: 1,
        cycle_value: 30,
        status: 1
      });
    }
    setModalOpen(true);
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      next_due_time: values.next_due_time?.toISOString() || null
    };
    try {
      if (current) {
        await updateMaintenancePlan(current.id, payload);
        message.success("更新成功");
      } else {
        await createMaintenancePlan(payload);
        message.success("创建成功");
      }
      setModalOpen(false);
      load();
    } catch {
      message.error("保存失败");
    }
  };

  const handleDelete = async (record) => {
    await deleteMaintenancePlan(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "计划号", dataIndex: "plan_no" },
    { title: "设备ID", dataIndex: "equipment_id" },
    { title: "计划类型", dataIndex: "plan_type" },
    { title: "周期类型", dataIndex: "cycle_type" },
    { title: "周期值", dataIndex: "cycle_value" },
    { title: "下次到期", dataIndex: "next_due_time" },
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
      title="设备维护计划"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建维护计划
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />
      <Modal
        open={modalOpen}
        title={current ? "编辑维护计划" : "新建维护计划"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="plan_no" label="计划号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="equipment_id" label="设备ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="plan_type" label="计划类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "保养" },
                { value: 2, label: "点检" },
                { value: 3, label: "校准" },
                { value: 4, label: "其他" }
              ]}
            />
          </Form.Item>
          <Form.Item name="cycle_type" label="周期类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "按时间" },
                { value: 2, label: "按运行小时" },
                { value: 3, label: "按次数" }
              ]}
            />
          </Form.Item>
          <Form.Item name="cycle_value" label="周期值" rules={[{ required: true }]}>
            <InputNumber min={1} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="next_due_time" label="下次到期时间">
            <DatePicker showTime />
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



