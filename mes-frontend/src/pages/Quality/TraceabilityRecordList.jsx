import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message, Descriptions } from "antd";
import {
  fetchTraceabilityRecords,
  createTraceabilityRecord,
  updateTraceabilityRecord,
  deleteTraceabilityRecord
} from "../../api/qualityApi";
import dayjs from "dayjs";

const traceTypeMap = {
  1: { text: "正向追溯", color: "blue" },
  2: { text: "反向追溯", color: "cyan" },
  3: { text: "双向追溯", color: "green" }
};

export default function TraceabilityRecordList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [detailModalOpen, setDetailModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchTraceabilityRecords({ page: 0, page_size: 50 });
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
        trace_date: record.trace_date
      });
    } else {
      form.setFieldsValue({
        trace_type: 1
      });
    }
  };

  const openDetailModal = (record) => {
    setCurrent(record);
    setDetailModalOpen(true);
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      trace_date: values.trace_date
    };
    try {
      if (current) {
        await updateTraceabilityRecord(current.id, payload);
        message.success("更新成功");
      } else {
        await createTraceabilityRecord(payload);
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
      await deleteTraceabilityRecord(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "追溯编号", dataIndex: "trace_no", width: 150 },
    {
      title: "追溯类型",
      dataIndex: "trace_type",
      width: 120,
      render: (v) => <Tag color={traceTypeMap[v]?.color}>{traceTypeMap[v]?.text || "未知"}</Tag>
    },
    { title: "物料ID", dataIndex: "material_id", width: 100 },
    { title: "批次号", dataIndex: "batch_no", width: 120 },
    { title: "序列号", dataIndex: "serial_no", width: 120 },
    { title: "生产订单号", dataIndex: "production_order_no", width: 150 },
    { title: "追溯日期", dataIndex: "trace_date", width: 120 },
    {
      title: "追溯结果",
      dataIndex: "trace_result",
      width: 200,
      render: (v) => v || "-"
    },
    {
      title: "操作",
      width: 200,
      fixed: "right",
      render: (_, record) => (
        <Space>
          <Button type="link" size="small" onClick={() => openDetailModal(record)}>
            详情
          </Button>
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
      title="质量追溯记录"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建追溯
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
        title={current ? "编辑追溯记录" : "新建追溯记录"}
        open={modalOpen}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="trace_no" label="追溯编号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="trace_type" label="追溯类型" rules={[{ required: true }]}>
            <Select>
              {Object.entries(traceTypeMap).map(([k, v]) => (
                <Select.Option key={k} value={parseInt(k)}>{v.text}</Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="material_id" label="物料ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="batch_no" label="批次号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="serial_no" label="序列号">
            <Input />
          </Form.Item>
          <Form.Item name="production_order_no" label="生产订单号">
            <Input />
          </Form.Item>
          <Form.Item name="trace_date" label="追溯日期" rules={[{ required: true }]}>
            <Input type="date" />
          </Form.Item>
          <Form.Item name="trace_reason" label="追溯原因">
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>

      <Modal
        title="追溯记录详情"
        open={detailModalOpen}
        onCancel={() => setDetailModalOpen(false)}
        footer={null}
        width={800}
      >
        {current && (
          <Descriptions column={2} bordered>
            <Descriptions.Item label="追溯编号">{current.trace_no}</Descriptions.Item>
            <Descriptions.Item label="追溯类型">
              <Tag color={traceTypeMap[current.trace_type]?.color}>
                {traceTypeMap[current.trace_type]?.text}
              </Tag>
            </Descriptions.Item>
            <Descriptions.Item label="物料ID">{current.material_id}</Descriptions.Item>
            <Descriptions.Item label="批次号">{current.batch_no}</Descriptions.Item>
            <Descriptions.Item label="序列号">{current.serial_no || "-"}</Descriptions.Item>
            <Descriptions.Item label="生产订单号">{current.production_order_no || "-"}</Descriptions.Item>
            <Descriptions.Item label="追溯日期">{current.trace_date}</Descriptions.Item>
            <Descriptions.Item label="追溯结果" span={2}>
              {current.trace_result || "-"}
            </Descriptions.Item>
          </Descriptions>
        )}
      </Modal>
    </Card>
  );
}


