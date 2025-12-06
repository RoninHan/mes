import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message, Statistic, Row, Col } from "antd";
import {
  fetchSupplierEvaluations,
  createSupplierEvaluation,
  updateSupplierEvaluation,
  deleteSupplierEvaluation
} from "../../api/qualityApi";
import dayjs from "dayjs";

const evaluationLevelMap = {
  A: { text: "优秀", color: "success" },
  B: { text: "良好", color: "processing" },
  C: { text: "合格", color: "warning" },
  D: { text: "不合格", color: "error" }
};

export default function SupplierEvaluationList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();
  const [stats, setStats] = useState({ total: 0, avgScore: 0, excellent: 0 });

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchSupplierEvaluations({ page: 0, page_size: 50 });
      setData(res.items || []);
      
      // 计算统计信息
      const items = res.items || [];
      const total = items.length;
      const avgScore = total > 0 
        ? items.reduce((sum, item) => sum + (item.total_score || 0), 0) / total 
        : 0;
      const excellent = items.filter(item => item.evaluation_level === "A").length;
      setStats({ total, avgScore, excellent });
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
        evaluation_date: record.evaluation_date
      });
    } else {
      form.setFieldsValue({
        evaluation_level: "C"
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      evaluation_date: values.evaluation_date,
      batch_qualified_rate: parseFloat(values.batch_qualified_rate) || 0,
      quantity_qualified_rate: parseFloat(values.quantity_qualified_rate) || 0,
      total_score: parseFloat(values.total_score) || 0
    };
    try {
      if (current) {
        await updateSupplierEvaluation(current.id, payload);
        message.success("更新成功");
      } else {
        await createSupplierEvaluation(payload);
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
      await deleteSupplierEvaluation(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "评估编号", dataIndex: "evaluation_no", width: 150 },
    { title: "供应商ID", dataIndex: "supplier_id", width: 100 },
    { title: "评估周期", dataIndex: "evaluation_period", width: 120 },
    { title: "评估日期", dataIndex: "evaluation_date", width: 120 },
    {
      title: "批次合格率",
      dataIndex: "batch_qualified_rate",
      width: 120,
      render: (v) => `${(v || 0).toFixed(2)}%`
    },
    {
      title: "数量合格率",
      dataIndex: "quantity_qualified_rate",
      width: 120,
      render: (v) => `${(v || 0).toFixed(2)}%`
    },
    {
      title: "综合得分",
      dataIndex: "total_score",
      width: 100,
      render: (v) => (v || 0).toFixed(2)
    },
    {
      title: "评估等级",
      dataIndex: "evaluation_level",
      width: 100,
      render: (v) => <Tag color={evaluationLevelMap[v]?.color}>{evaluationLevelMap[v]?.text || v}</Tag>
    },
    {
      title: "审核状态",
      dataIndex: "is_approved",
      width: 100,
      render: (v) => v === 1 ? <Tag color="success">已审核</Tag> : <Tag color="default">待审核</Tag>
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
      title="供应商质量评估"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建评估
        </Button>
      }
    >
      <Row gutter={16} style={{ marginBottom: 16 }}>
        <Col span={8}>
          <Statistic title="评估总数" value={stats.total} />
        </Col>
        <Col span={8}>
          <Statistic title="平均得分" value={stats.avgScore.toFixed(2)} suffix="分" />
        </Col>
        <Col span={8}>
          <Statistic title="优秀供应商" value={stats.excellent} suffix="家" />
        </Col>
      </Row>

      <Table
        columns={columns}
        dataSource={data}
        loading={loading}
        rowKey="id"
        scroll={{ x: 1200 }}
        pagination={{ pageSize: 50 }}
      />

      <Modal
        title={current ? "编辑供应商评估" : "新建供应商评估"}
        open={modalOpen}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="evaluation_no" label="评估编号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="supplier_id" label="供应商ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="evaluation_period" label="评估周期" rules={[{ required: true }]}>
            <Input placeholder="如：2024-Q1" />
          </Form.Item>
          <Form.Item name="evaluation_date" label="评估日期" rules={[{ required: true }]}>
            <Input type="date" />
          </Form.Item>
          <Form.Item name="batch_qualified_rate" label="批次合格率（%）">
            <Input type="number" step="0.01" />
          </Form.Item>
          <Form.Item name="quantity_qualified_rate" label="数量合格率（%）">
            <Input type="number" step="0.01" />
          </Form.Item>
          <Form.Item name="total_score" label="综合得分">
            <Input type="number" step="0.01" />
          </Form.Item>
          <Form.Item name="evaluation_level" label="评估等级">
            <Select>
              {Object.entries(evaluationLevelMap).map(([k, v]) => (
                <Select.Option key={k} value={k}>{v.text}</Select.Option>
              ))}
            </Select>
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


