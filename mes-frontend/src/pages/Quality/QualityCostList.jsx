import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message, Row, Col, Statistic } from "antd";
import {
  fetchQualityCosts,
  createQualityCost,
  updateQualityCost,
  deleteQualityCost
} from "../../api/qualityApi";
import dayjs from "dayjs";

const costCategoryMap = {
  1: { text: "预防成本", color: "blue" },
  2: { text: "鉴定成本", color: "cyan" },
  3: { text: "内部损失成本", color: "orange" },
  4: { text: "外部损失成本", color: "red" }
};

export default function QualityCostList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();
  const [stats, setStats] = useState({ total: 0, byCategory: {} });

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchQualityCosts({ page: 0, page_size: 50 });
      setData(res.items || []);
      
      // 计算统计信息
      const items = res.items || [];
      const total = items.reduce((sum, item) => sum + (item.cost_amount || 0), 0);
      const byCategory = {};
      items.forEach(item => {
        const category = costCategoryMap[item.cost_category]?.text || "其他";
        byCategory[category] = (byCategory[category] || 0) + (item.cost_amount || 0);
      });
      setStats({ total, byCategory });
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
        cost_date: record.cost_date,
        cost_amount: record.cost_amount
      });
    } else {
      form.setFieldsValue({
        cost_category: 1
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      cost_date: values.cost_date,
      cost_amount: parseFloat(values.cost_amount) || 0
    };
    try {
      if (current) {
        await updateQualityCost(current.id, payload);
        message.success("更新成功");
      } else {
        await createQualityCost(payload);
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
      await deleteQualityCost(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "成本编号", dataIndex: "cost_no", width: 150 },
    { title: "成本周期", dataIndex: "cost_period", width: 120 },
    { title: "成本日期", dataIndex: "cost_date", width: 120 },
    {
      title: "成本类别",
      dataIndex: "cost_category",
      width: 120,
      render: (v) => <Tag color={costCategoryMap[v]?.color}>{costCategoryMap[v]?.text || "未知"}</Tag>
    },
    { title: "成本类型", dataIndex: "cost_type", width: 150 },
    { title: "成本项目", dataIndex: "cost_item", width: 200 },
    {
      title: "成本金额",
      dataIndex: "cost_amount",
      width: 120,
      render: (v) => `¥${(v || 0).toFixed(2)}`
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
      title="质量成本分析"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建成本记录
        </Button>
      }
    >
      <Row gutter={16} style={{ marginBottom: 16 }}>
        <Col span={6}>
          <Statistic title="总成本" value={stats.total.toFixed(2)} prefix="¥" />
        </Col>
        {Object.entries(stats.byCategory).map(([category, amount]) => (
          <Col span={6} key={category}>
            <Statistic title={category} value={amount.toFixed(2)} prefix="¥" />
          </Col>
        ))}
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
        title={current ? "编辑质量成本" : "新建质量成本"}
        open={modalOpen}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="cost_no" label="成本编号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="cost_period" label="成本周期" rules={[{ required: true }]}>
            <Input placeholder="如：2024-Q1" />
          </Form.Item>
          <Form.Item name="cost_date" label="成本日期" rules={[{ required: true }]}>
            <Input type="date" />
          </Form.Item>
          <Form.Item name="cost_category" label="成本类别" rules={[{ required: true }]}>
            <Select>
              {Object.entries(costCategoryMap).map(([k, v]) => (
                <Select.Option key={k} value={parseInt(k)}>{v.text}</Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="cost_type" label="成本类型" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="cost_item" label="成本项目" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="cost_amount" label="成本金额" rules={[{ required: true }]}>
            <Input type="number" step="0.01" prefix="¥" />
          </Form.Item>
          <Form.Item name="cost_description" label="成本描述">
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


