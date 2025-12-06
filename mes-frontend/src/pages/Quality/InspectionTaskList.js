import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message } from "antd";
import dayjs from "dayjs";
import {
  fetchInspectionTasks,
  createInspectionTask,
  updateInspectionTask,
  deleteInspectionTask
} from "../../api/qualityApi";

const inspectionTypeMap = {
  1: { text: "IQC来料检验", color: "blue" },
  2: { text: "IPQC过程检验", color: "cyan" },
  3: { text: "FQC成品检验", color: "green" },
  4: { text: "OQC出货检验", color: "orange" },
  5: { text: "委外检验", color: "purple" }
};

const taskStatusMap = {
  1: { text: "待检验", color: "default" },
  2: { text: "检验中", color: "processing" },
  3: { text: "已完成", color: "success" },
  4: { text: "已取消", color: "error" }
};

const inspectionResultMap = {
  1: { text: "合格", color: "success" },
  2: { text: "不合格", color: "error" },
  3: { text: "让步接收", color: "warning" },
  4: { text: "待定", color: "default" }
};

export default function InspectionTaskList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();
  const [filters, setFilters] = useState({});

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchInspectionTasks({ ...filters, page: 0, page_size: 50 });
      setData(res.items || []);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
  }, [filters]);

  const openModal = (record) => {
    setCurrent(record || null);
    setModalOpen(true);
    form.resetFields();
    if (record) {
      form.setFieldsValue({
        ...record,
        inspection_quantity: record.inspection_quantity,
        plan_start_time: record.plan_start_time,
        plan_end_time: record.plan_end_time
      });
    } else {
      form.setFieldsValue({
        inspection_type: 1,
        source_type: 1,
        priority: 3,
        task_status: 1
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      inspection_quantity: parseFloat(values.inspection_quantity) || 0,
      plan_start_time: values.plan_start_time ? new Date(values.plan_start_time).toISOString() : null,
      plan_end_time: values.plan_end_time ? new Date(values.plan_end_time).toISOString() : null
    };
    try {
      if (current) {
        await updateInspectionTask(current.id, payload);
        message.success("更新成功");
      } else {
        await createInspectionTask(payload);
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
      await deleteInspectionTask(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "任务编号", dataIndex: "task_no", width: 150 },
    {
      title: "检验类型",
      dataIndex: "inspection_type",
      width: 120,
      render: (v) => <Tag color={inspectionTypeMap[v]?.color}>{inspectionTypeMap[v]?.text || "未知"}</Tag>
    },
    { title: "来源单号", dataIndex: "source_order_no", width: 150 },
    { title: "物料ID", dataIndex: "material_id", width: 100 },
    { title: "批次号", dataIndex: "batch_no", width: 120 },
    {
      title: "任务状态",
      dataIndex: "task_status",
      width: 100,
      render: (v) => <Tag color={taskStatusMap[v]?.color}>{taskStatusMap[v]?.text || "未知"}</Tag>
    },
    {
      title: "检验结果",
      dataIndex: "inspection_result",
      width: 100,
      render: (v) => v ? <Tag color={inspectionResultMap[v]?.color}>{inspectionResultMap[v]?.text || "未知"}</Tag> : "-"
    },
    { title: "送检数量", dataIndex: "inspection_quantity", width: 100 },
    { title: "合格数量", dataIndex: "qualified_quantity", width: 100 },
    { title: "单位", dataIndex: "unit", width: 80 },
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
      title="质检任务管理"
      extra={
        <Space>
          <Select
            placeholder="检验类型"
            allowClear
            style={{ width: 150 }}
            onChange={(v) => setFilters({ ...filters, inspection_type: v })}
          >
            {Object.entries(inspectionTypeMap).map(([k, v]) => (
              <Select.Option key={k} value={parseInt(k)}>{v.text}</Select.Option>
            ))}
          </Select>
          <Select
            placeholder="任务状态"
            allowClear
            style={{ width: 120 }}
            onChange={(v) => setFilters({ ...filters, task_status: v })}
          >
            {Object.entries(taskStatusMap).map(([k, v]) => (
              <Select.Option key={k} value={parseInt(k)}>{v.text}</Select.Option>
            ))}
          </Select>
          <Button type="primary" onClick={() => openModal(null)}>
            新建任务
          </Button>
        </Space>
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
        title={current ? "编辑质检任务" : "新建质检任务"}
        open={modalOpen}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="task_no" label="任务编号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="inspection_type" label="检验类型" rules={[{ required: true }]}>
            <Select>
              {Object.entries(inspectionTypeMap).map(([k, v]) => (
                <Select.Option key={k} value={parseInt(k)}>{v.text}</Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="source_type" label="来源类型" rules={[{ required: true }]}>
            <Select>
              <Select.Option value={1}>入库单</Select.Option>
              <Select.Option value={2}>生产工单</Select.Option>
              <Select.Option value={3}>完工单</Select.Option>
              <Select.Option value={4}>出库单</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="source_order_no" label="来源单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="material_id" label="物料ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="batch_no" label="批次号">
            <Input />
          </Form.Item>
          <Form.Item name="inspection_quantity" label="送检数量" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="unit" label="单位" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="priority" label="优先级">
            <Select>
              <Select.Option value={1}>紧急</Select.Option>
              <Select.Option value={2}>高</Select.Option>
              <Select.Option value={3}>普通</Select.Option>
              <Select.Option value={4}>低</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}

