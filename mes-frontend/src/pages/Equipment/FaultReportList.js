import React, { useEffect, useState } from "react";
import { Button, Card, DatePicker, Form, Input, InputNumber, Modal, Select, Space, Table, Tag, message } from "antd";
import dayjs from "dayjs";
import {
  fetchFaultReports,
  createFaultReport,
  updateFaultReport,
  deleteFaultReport
} from "../../api/equipmentApi";

const statusMap = {
  1: { text: "待响应", color: "default" },
  2: { text: "处理中", color: "blue" },
  3: { text: "已恢复", color: "green" },
  4: { text: "已关闭", color: "red" }
};

export default function FaultReportList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchFaultReports({ page: 0, page_size: 50 });
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
        occur_time: record.occur_time ? dayjs(record.occur_time) : null,
        report_time: record.report_time ? dayjs(record.report_time) : null
      });
    } else {
      form.setFieldsValue({
        fault_level: 1,
        status: 1,
        occur_time: dayjs(),
        report_time: dayjs()
      });
    }
    setModalOpen(true);
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      occur_time: values.occur_time.toISOString(),
      report_time: values.report_time?.toISOString() || null
    };
    try {
      if (current) {
        await updateFaultReport(current.id, payload);
        message.success("更新成功");
      } else {
        await createFaultReport(payload);
        message.success("创建成功");
      }
      setModalOpen(false);
      load();
    } catch {
      message.error("保存失败");
    }
  };

  const handleDelete = async (record) => {
    await deleteFaultReport(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "故障单号", dataIndex: "fault_no" },
    { title: "设备ID", dataIndex: "equipment_id" },
    { title: "等级", dataIndex: "fault_level" },
    { title: "发生时间", dataIndex: "occur_time" },
    { title: "报修时间", dataIndex: "report_time" },
    {
      title: "状态",
      dataIndex: "status",
      render: (v) => <Tag color={statusMap[v]?.color}>{statusMap[v]?.text || "未知"}</Tag>
    },
    { title: "描述", dataIndex: "description", ellipsis: true },
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
      title="设备故障报修"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新增故障
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />
      <Modal
        open={modalOpen}
        title={current ? "编辑故障" : "新增故障"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="fault_no" label="故障单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="equipment_id" label="设备ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="fault_level" label="故障等级" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "一般" },
                { value: 2, label: "严重" },
                { value: 3, label: "致命" }
              ]}
            />
          </Form.Item>
          <Form.Item name="occur_time" label="发生时间" rules={[{ required: true }]}>
            <DatePicker showTime />
          </Form.Item>
          <Form.Item name="report_time" label="报修时间">
            <DatePicker showTime />
          </Form.Item>
          <Form.Item name="reporter_id" label="报修人ID">
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="description" label="故障描述">
            <Input.TextArea rows={3} />
          </Form.Item>
          <Form.Item name="status" label="状态" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "待响应" },
                { value: 2, label: "处理中" },
                { value: 3, label: "已恢复" },
                { value: 4, label: "已关闭" }
              ]}
            />
          </Form.Item>
          <Form.Item name="root_cause" label="根因分析">
            <Input.TextArea rows={3} />
          </Form.Item>
          <Form.Item name="solution" label="处理措施">
            <Input.TextArea rows={3} />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={2} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}



