import React, { useEffect, useState } from "react";
import { Button, Card, DatePicker, Form, Input, InputNumber, Modal, Select, Space, Table, Tag, message } from "antd";
import dayjs from "dayjs";
import {
  fetchMaintenanceTasks,
  createMaintenanceTask,
  updateMaintenanceTask,
  deleteMaintenanceTask
} from "../../api/equipmentApi";

const statusMap = {
  1: { text: "待执行", color: "default" },
  2: { text: "执行中", color: "blue" },
  3: { text: "已完成", color: "green" },
  4: { text: "已关闭", color: "red" }
};

export default function MaintenanceTaskList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchMaintenanceTasks({ page: 0, page_size: 50 });
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
        scheduled_time: record.scheduled_time ? dayjs(record.scheduled_time) : null,
        start_time: record.start_time ? dayjs(record.start_time) : null,
        end_time: record.end_time ? dayjs(record.end_time) : null
      });
    } else {
      form.setFieldsValue({
        task_type: 1,
        status: 1
      });
    }
    setModalOpen(true);
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      scheduled_time: values.scheduled_time?.toISOString() || null,
      start_time: values.start_time?.toISOString() || null,
      end_time: values.end_time?.toISOString() || null
    };
    try {
      if (current) {
        await updateMaintenanceTask(current.id, payload);
        message.success("更新成功");
      } else {
        await createMaintenanceTask(payload);
        message.success("创建成功");
      }
      setModalOpen(false);
      load();
    } catch {
      message.error("保存失败");
    }
  };

  const handleDelete = async (record) => {
    await deleteMaintenanceTask(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "任务号", dataIndex: "task_no" },
    { title: "计划ID", dataIndex: "plan_id" },
    { title: "设备ID", dataIndex: "equipment_id" },
    { title: "任务类型", dataIndex: "task_type" },
    { title: "计划时间", dataIndex: "scheduled_time" },
    { title: "开始时间", dataIndex: "start_time" },
    { title: "结束时间", dataIndex: "end_time" },
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
      title="维护任务"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建任务
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />
      <Modal
        open={modalOpen}
        title={current ? "编辑维护任务" : "新建维护任务"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="task_no" label="任务号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="plan_id" label="计划ID">
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="equipment_id" label="设备ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="task_type" label="任务类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "保养" },
                { value: 2, label: "点检" },
                { value: 3, label: "校准" },
                { value: 4, label: "临时" }
              ]}
            />
          </Form.Item>
          <Form.Item name="scheduled_time" label="计划时间">
            <DatePicker showTime />
          </Form.Item>
          <Form.Item name="start_time" label="开始时间">
            <DatePicker showTime />
          </Form.Item>
          <Form.Item name="end_time" label="结束时间">
            <DatePicker showTime />
          </Form.Item>
          <Form.Item name="status" label="状态" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "待执行" },
                { value: 2, label: "执行中" },
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



