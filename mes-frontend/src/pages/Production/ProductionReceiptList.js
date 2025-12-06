import React, { useEffect, useState } from "react";
import {
  Button,
  Card,
  DatePicker,
  Form,
  Input,
  InputNumber,
  Modal,
  Space,
  Table,
  message
} from "antd";
import dayjs from "dayjs";
import {
  fetchProductionReceipts,
  fetchProductionReceipt,
  createProductionReceipt,
  updateProductionReceipt,
  deleteProductionReceipt
} from "../../api/productionApi";

export default function ProductionReceiptList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [currentId, setCurrentId] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchProductionReceipts({ page: 0, page_size: 50 });
      setData(res.items || []);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
  }, []);

  const openNew = () => {
    setCurrentId(null);
    form.resetFields();
    form.setFieldsValue({
      receipt_type: 1,
      receipt_date: dayjs(),
      quantity: 0,
      qualified_quantity: 0,
      unqualified_quantity: 0
    });
    setModalOpen(true);
  };

  const openEdit = async (record) => {
    const full = await fetchProductionReceipt(record.id);
    setCurrentId(record.id);
    form.setFieldsValue({
      ...full,
      receipt_date: full.receipt_date ? dayjs(full.receipt_date) : null
    });
    setModalOpen(true);
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      receipt_date: values.receipt_date?.format("YYYY-MM-DD") || null
    };
    try {
      if (currentId) {
        await updateProductionReceipt(currentId, payload);
        message.success("更新成功");
      } else {
        await createProductionReceipt(payload);
        message.success("创建成功");
      }
      setModalOpen(false);
      load();
    } catch {
      message.error("保存失败");
    }
  };

  const handleDelete = async (record) => {
    await deleteProductionReceipt(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "完工单号", dataIndex: "receipt_no" },
    { title: "生产订单ID", dataIndex: "production_order_id" },
    { title: "工单ID", dataIndex: "work_order_id" },
    { title: "物料ID", dataIndex: "material_id" },
    { title: "仓库ID", dataIndex: "warehouse_id" },
    { title: "库位ID", dataIndex: "location_id" },
    { title: "入库类型", dataIndex: "receipt_type" },
    { title: "入库日期", dataIndex: "receipt_date" },
    {
      title: "数量",
      dataIndex: "quantity",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    { title: "合格数", dataIndex: "qualified_quantity" },
    { title: "不合格数", dataIndex: "unqualified_quantity" },
    { title: "单位", dataIndex: "unit" },
    {
      title: "操作",
      render: (_, record) => (
        <Space>
          <Button type="link" onClick={() => openEdit(record)}>
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
      title="完工入库"
      extra={
        <Button type="primary" onClick={openNew}>
          新建完工入库
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />
      <Modal
        open={modalOpen}
        title={currentId ? "编辑完工入库" : "新建完工入库"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="receipt_no" label="完工单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item
            name="production_order_id"
            label="生产订单ID"
            rules={[{ required: true }]}
          >
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="work_order_id" label="工单ID">
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="material_id" label="物料ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="warehouse_id" label="仓库ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="location_id" label="库位ID">
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item
            name="receipt_type"
            label="入库类型"
            rules={[{ required: true }]}
          >
            <InputNumber min={1} max={3} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="receipt_date" label="入库日期">
            <DatePicker />
          </Form.Item>
          <Form.Item name="quantity" label="数量" rules={[{ required: true }]}>
            <InputNumber min={0} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item
            name="qualified_quantity"
            label="合格数量"
            rules={[{ required: true }]}
          >
            <InputNumber min={0} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item
            name="unqualified_quantity"
            label="不合格数量"
            rules={[{ required: true }]}
          >
            <InputNumber min={0} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="unit" label="单位" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


