import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message } from "antd";
import dayjs from "dayjs";
import {
  fetchInspectionReports,
  createInspectionReport,
  updateInspectionReport,
  deleteInspectionReport
} from "../../api/qualityApi";

const reportStatusMap = {
  1: { text: "待审核", color: "default" },
  2: { text: "已审核", color: "success" },
  3: { text: "已归档", color: "blue" }
};

const inspectionResultMap = {
  1: { text: "合格", color: "success" },
  2: { text: "不合格", color: "error" },
  3: { text: "让步接收", color: "warning" },
  4: { text: "待定", color: "default" }
};

export default function InspectionReportList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchInspectionReports({ page: 0, page_size: 50 });
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
        inspection_date: record.inspection_date,
        inspection_time: record.inspection_time
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      inspection_quantity: parseFloat(values.inspection_quantity) || 0,
      sample_quantity: parseFloat(values.sample_quantity) || 0,
      qualified_quantity: parseFloat(values.qualified_quantity) || 0,
      unqualified_quantity: parseFloat(values.unqualified_quantity) || 0,
      inspection_date: values.inspection_date,
      inspection_time: values.inspection_time ? new Date(values.inspection_time).toISOString() : new Date().toISOString()
    };
    try {
      if (current) {
        await updateInspectionReport(current.id, payload);
        message.success("更新成功");
      } else {
        await createInspectionReport(payload);
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
      await deleteInspectionReport(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "报告编号", dataIndex: "report_no", width: 150 },
    { title: "任务ID", dataIndex: "task_id", width: 100 },
    { title: "物料ID", dataIndex: "material_id", width: 100 },
    { title: "批次号", dataIndex: "batch_no", width: 120 },
    { title: "检验日期", dataIndex: "inspection_date", width: 120 },
    {
      title: "检验结果",
      dataIndex: "inspection_result",
      width: 100,
      render: (v) => <Tag color={inspectionResultMap[v]?.color}>{inspectionResultMap[v]?.text || "未知"}</Tag>
    },
    { title: "送检数量", dataIndex: "inspection_quantity", width: 100 },
    { title: "合格数量", dataIndex: "qualified_quantity", width: 100 },
    {
      title: "合格率",
      dataIndex: "qualified_rate",
      width: 100,
      render: (v) => `${v.toFixed(2)}%`
    },
    {
      title: "报告状态",
      dataIndex: "report_status",
      width: 100,
      render: (v) => <Tag color={reportStatusMap[v]?.color}>{reportStatusMap[v]?.text || "未知"}</Tag>
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
      title="质检报告管理"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建报告
        </Button>
      }
    >
      <Table
        columns={columns}
        dataSource={data}
        loading={loading}
        rowKey="id"
        scroll={{ x: 1200 }}
        pagination={{ pageSize: 50 }}
      />

      <Modal
        title={current ? "编辑质检报告" : "新建质检报告"}
        open={modalOpen}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="report_no" label="报告编号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="task_id" label="任务ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="material_id" label="物料ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="inspection_date" label="检验日期" rules={[{ required: true }]}>
            <Input type="date" />
          </Form.Item>
          <Form.Item name="inspection_quantity" label="送检数量" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="sample_quantity" label="抽样数量" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="qualified_quantity" label="合格数量" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="unqualified_quantity" label="不合格数量" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="inspection_result" label="检验结果" rules={[{ required: true }]}>
            <Select>
              {Object.entries(inspectionResultMap).map(([k, v]) => (
                <Select.Option key={k} value={parseInt(k)}>{v.text}</Select.Option>
              ))}
            </Select>
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}

