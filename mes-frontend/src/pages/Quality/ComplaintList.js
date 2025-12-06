import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message } from "antd";
import { fetchComplaints, createComplaint, updateComplaint, deleteComplaint } from "../../api/qualityApi";

const complaintStatusMap = {
  1: { text: "待处理", color: "default" },
  2: { text: "处理中", color: "processing" },
  3: { text: "待验证", color: "warning" },
  4: { text: "已关闭", color: "success" }
};

const complaintTypeMap = {
  1: { text: "质量问题", color: "red" },
  2: { text: "交期问题", color: "orange" },
  3: { text: "服务问题", color: "blue" },
  4: { text: "包装问题", color: "cyan" },
  5: { text: "其他", color: "default" }
};

export default function ComplaintList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchComplaints({ page: 0, page_size: 50 });
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
        complaint_date: record.complaint_date
      });
    } else {
      form.setFieldsValue({
        complaint_type: 1,
        complaint_level: 3,
        complaint_status: 1
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      complaint_date: values.complaint_date,
      complaint_time: new Date().toISOString()
    };
    try {
      if (current) {
        await updateComplaint(current.id, payload);
        message.success("更新成功");
      } else {
        await createComplaint(payload);
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
      await deleteComplaint(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "投诉编号", dataIndex: "complaint_no", width: 150 },
    { title: "客户ID", dataIndex: "customer_id", width: 100 },
    { title: "物料ID", dataIndex: "material_id", width: 100 },
    { title: "批次号", dataIndex: "batch_no", width: 120 },
    {
      title: "投诉类型",
      dataIndex: "complaint_type",
      width: 100,
      render: (v) => <Tag color={complaintTypeMap[v]?.color}>{complaintTypeMap[v]?.text || "未知"}</Tag>
    },
    { title: "投诉日期", dataIndex: "complaint_date", width: 120 },
    {
      title: "投诉状态",
      dataIndex: "complaint_status",
      width: 100,
      render: (v) => <Tag color={complaintStatusMap[v]?.color}>{complaintStatusMap[v]?.text || "未知"}</Tag>
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
      title="客户投诉管理"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建投诉
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
        title={current ? "编辑投诉" : "新建投诉"}
        open={modalOpen}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="complaint_no" label="投诉编号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="customer_id" label="客户ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="material_id" label="物料ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="complaint_type" label="投诉类型" rules={[{ required: true }]}>
            <Select>
              {Object.entries(complaintTypeMap).map(([k, v]) => (
                <Select.Option key={k} value={parseInt(k)}>{v.text}</Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="complaint_date" label="投诉日期" rules={[{ required: true }]}>
            <Input type="date" />
          </Form.Item>
          <Form.Item name="complaint_description" label="投诉描述" rules={[{ required: true }]}>
            <Input.TextArea rows={4} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


