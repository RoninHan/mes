import React, { useEffect, useState } from "react";
import { Button, Card, DatePicker, Form, Input, InputNumber, Modal, Select, Space, Table, Tag, message } from "antd";
import dayjs from "dayjs";
import {
  fetchInspections,
  createInspection,
  updateInspection,
  deleteInspection
} from "../../api/equipmentApi";

const resultMap = {
  1: { text: "正常", color: "green" },
  2: { text: "异常", color: "red" }
};

export default function InspectionList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchInspections({ page: 0, page_size: 50 });
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
        inspection_time: record.inspection_time ? dayjs(record.inspection_time) : null,
        itemsJson: record.items ? JSON.stringify(record.items, null, 2) : ""
      });
    } else {
      form.setFieldsValue({
        inspection_type: 1,
        result: 1,
        inspection_time: dayjs(),
        itemsJson: '[{ "item": "外观", "result": "OK" }]'
      });
    }
    setModalOpen(true);
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    let itemsParsed = null;
    if (values.itemsJson) {
      try {
        itemsParsed = JSON.parse(values.itemsJson);
      } catch {
        message.error("检查项目需为合法 JSON");
        return;
      }
    }
    const payload = {
      inspection_no: values.inspection_no,
      equipment_id: values.equipment_id,
      inspection_type: values.inspection_type,
      inspection_time: values.inspection_time?.toISOString() || null,
      inspector_id: values.inspector_id,
      result: values.result,
      items: itemsParsed,
      remark: values.remark
    };
    try {
      if (current) {
        await updateInspection(current.id, payload);
        message.success("更新成功");
      } else {
        await createInspection(payload);
        message.success("创建成功");
      }
      setModalOpen(false);
      load();
    } catch {
      message.error("保存失败");
    }
  };

  const handleDelete = async (record) => {
    await deleteInspection(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "点检单号", dataIndex: "inspection_no" },
    { title: "设备ID", dataIndex: "equipment_id" },
    { title: "类型", dataIndex: "inspection_type" },
    { title: "时间", dataIndex: "inspection_time" },
    { title: "点检人ID", dataIndex: "inspector_id" },
    {
      title: "结果",
      dataIndex: "result",
      render: (v) => <Tag color={resultMap[v]?.color}>{resultMap[v]?.text || "未知"}</Tag>
    },
    { title: "备注", dataIndex: "remark", ellipsis: true },
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
      title="设备点检记录"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建点检
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />
      <Modal
        open={modalOpen}
        title={current ? "编辑点检记录" : "新建点检记录"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={720}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="inspection_no" label="点检单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="equipment_id" label="设备ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="inspection_type" label="点检类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "点检" },
                { value: 2, label: "巡检" },
                { value: 3, label: "专项" }
              ]}
            />
          </Form.Item>
          <Form.Item name="inspection_time" label="点检时间" rules={[{ required: true }]}>
            <DatePicker showTime />
          </Form.Item>
          <Form.Item name="inspector_id" label="点检人ID">
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="result" label="结果" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "正常" },
                { value: 2, label: "异常" }
              ]}
            />
          </Form.Item>
          <Form.Item name="itemsJson" label="检查项目（JSON）">
            <Input.TextArea rows={6} />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}



