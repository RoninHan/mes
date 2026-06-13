import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, DatePicker, InputNumber, message } from "antd";
import dayjs from "dayjs";
import {
  fetchProductionReports,
  createProductionReport,
  updateProductionReport,
  deleteProductionReport
} from "../../api/productionApi";

export default function ProductionReportList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchProductionReports({ page: 0, page_size: 50 });
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
        report_date: dayjs(record.report_date)
      });
    } else {
      form.setFieldsValue({
        report_type: 1,
        report_date: dayjs()
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      report_date: values.report_date.format("YYYY-MM-DD")
    };
    if (current) {
      await updateProductionReport(current.id, payload);
      message.success("更新成功");
    } else {
      await createProductionReport(payload);
      message.success("创建成功");
    }
    setModalOpen(false);
    load();
  };

  const handleDelete = async (record) => {
    await deleteProductionReport(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "报工单号", dataIndex: "report_no" },
    { title: "工单ID", dataIndex: "work_order_id" },
    { title: "类型", dataIndex: "report_type" },
    { title: "报工日期", dataIndex: "report_date" },
    {
      title: "报工数量",
      dataIndex: "report_quantity",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    { title: "合格数量", dataIndex: "qualified_quantity" },
    { title: "不合格数量", dataIndex: "unqualified_quantity" },
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
      title="生产报工"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建报工
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
      <Modal
        open={modalOpen}
        title={current ? "编辑报工" : "新建报工"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="report_no" label="报工单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="work_order_id" label="工单ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="production_order_id" label="生产订单ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="process_id" label="工序ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="material_id" label="物料ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="report_type" label="报工类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "开工" },
                { value: 2, label: "完工" },
                { value: 3, label: "进度" },
                { value: 4, label: "返工" }
              ]}
            />
          </Form.Item>
          <Form.Item name="report_date" label="报工日期" rules={[{ required: true }]}>
            <DatePicker />
          </Form.Item>
          <Form.Item name="report_quantity" label="报工数量" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} min={0} />
          </Form.Item>
          <Form.Item name="qualified_quantity" label="合格数量" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} min={0} />
          </Form.Item>
          <Form.Item name="unqualified_quantity" label="不合格数量" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} min={0} />
          </Form.Item>
          <Form.Item name="operator_id" label="操作员ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


