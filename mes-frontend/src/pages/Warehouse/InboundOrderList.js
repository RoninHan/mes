import React, { useEffect, useState } from "react";
import { Button, Card, DatePicker, Form, Input, InputNumber, Modal, Select, Space, Table, Tag, message } from "antd";
import dayjs from "dayjs";
import {
  fetchInboundOrders,
  fetchInboundOrder,
  createInboundOrder,
  updateInboundOrder,
  deleteInboundOrder
} from "../../api/warehouseApi";

const statusMap = {
  1: { text: "待入库", color: "default" },
  2: { text: "部分入库", color: "gold" },
  3: { text: "已完成", color: "green" },
  4: { text: "已取消", color: "red" }
};

export default function InboundOrderList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [currentId, setCurrentId] = useState(null);
  const [form] = Form.useForm();
  const [detailForm] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchInboundOrders({ page: 0, page_size: 50 });
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
    detailForm.setFieldsValue({ details: [{ material_id: null, plan_quantity: 0, unit: "EA", unit_price: 0 }] });
    form.setFieldsValue({
      inbound_type: 1,
      plan_inbound_date: dayjs()
    });
    setModalOpen(true);
  };

  const openEdit = async (record) => {
    const full = await fetchInboundOrder(record.id);
    setCurrentId(record.id);
    form.setFieldsValue({
      inbound_no: full.header.inbound_no,
      inbound_type: full.header.inbound_type,
      warehouse_id: full.header.warehouse_id,
      supplier_id: full.header.supplier_id,
      plan_inbound_date: full.header.plan_inbound_date ? dayjs(full.header.plan_inbound_date) : null,
      remark: full.header.remark
    });
    detailForm.setFieldsValue({
      details: full.details.map((d) => ({
        material_id: d.material_id,
        location_id: d.location_id,
        batch_no: d.batch_no,
        plan_quantity: d.plan_quantity,
        unit: d.unit,
        unit_price: d.unit_price
      }))
    });
    setModalOpen(true);
  };

  const handleOk = async () => {
    const header = await form.validateFields();
    const { details } = await detailForm.validateFields();
    const payload = {
      ...header,
      plan_inbound_date: header.plan_inbound_date?.format("YYYY-MM-DD") || null,
      details
    };
    if (currentId) {
      await updateInboundOrder(currentId, payload);
      message.success("更新成功");
    } else {
      await createInboundOrder(payload);
      message.success("创建成功");
    }
    setModalOpen(false);
    load();
  };

  const handleDelete = async (record) => {
    await deleteInboundOrder(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "入库单号", dataIndex: "inbound_no" },
    { title: "类型", dataIndex: "inbound_type" },
    { title: "仓库ID", dataIndex: "warehouse_id" },
    { title: "供应商ID", dataIndex: "supplier_id" },
    { title: "计划日期", dataIndex: "plan_inbound_date" },
    { title: "实际日期", dataIndex: "actual_inbound_date" },
    {
      title: "总数量",
      dataIndex: "total_quantity",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    {
      title: "状态",
      dataIndex: "order_status",
      render: (v) => <Tag color={statusMap[v]?.color}>{statusMap[v]?.text || "未知"}</Tag>
    },
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
      title="入库单"
      extra={
        <Button type="primary" onClick={openNew}>
          新建入库单
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
      <Modal
        open={modalOpen}
        title={currentId ? "编辑入库单" : "新建入库单"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={900}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="inbound_no" label="入库单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="inbound_type" label="入库类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "采购入库" },
                { value: 2, label: "生产入库" },
                { value: 3, label: "退货入库" },
                { value: 4, label: "调拨入库" },
                { value: 5, label: "其他入库" }
              ]}
            />
          </Form.Item>
          <Form.Item name="warehouse_id" label="仓库ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="supplier_id" label="供应商ID">
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="plan_inbound_date" label="计划入库日期">
            <DatePicker />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={2} />
          </Form.Item>
        </Form>

        <Form form={detailForm} layout="vertical">
          <Form.List name="details">
            {(fields, { add, remove }) => (
              <>
                <Table
                  pagination={false}
                  rowKey={(record, index) => index}
                  dataSource={fields}
                  columns={[
                    {
                      title: "物料ID",
                      render: (_, field, index) => (
                        <Form.Item
                          name={[field.name, "material_id"]}
                          rules={[{ required: true }]}
                          style={{ marginBottom: 0 }}
                        >
                          <InputNumber style={{ width: "100%" }} />
                        </Form.Item>
                      )
                    },
                    {
                      title: "库位ID",
                      render: (_, field) => (
                        <Form.Item
                          name={[field.name, "location_id"]}
                          style={{ marginBottom: 0 }}
                        >
                          <InputNumber style={{ width: "100%" }} />
                        </Form.Item>
                      )
                    },
                    {
                      title: "批次号",
                      render: (_, field) => (
                        <Form.Item
                          name={[field.name, "batch_no"]}
                          style={{ marginBottom: 0 }}
                        >
                          <Input />
                        </Form.Item>
                      )
                    },
                    {
                      title: "计划数量",
                      render: (_, field) => (
                        <Form.Item
                          name={[field.name, "plan_quantity"]}
                          rules={[{ required: true }]}
                          style={{ marginBottom: 0 }}
                        >
                          <InputNumber min={0} style={{ width: "100%" }} />
                        </Form.Item>
                      )
                    },
                    {
                      title: "单位",
                      render: (_, field) => (
                        <Form.Item
                          name={[field.name, "unit"]}
                          rules={[{ required: true }]}
                          style={{ marginBottom: 0 }}
                        >
                          <Input />
                        </Form.Item>
                      )
                    },
                    {
                      title: "单价",
                      render: (_, field) => (
                        <Form.Item
                          name={[field.name, "unit_price"]}
                          rules={[{ required: true }]}
                          style={{ marginBottom: 0 }}
                        >
                          <InputNumber min={0} style={{ width: "100%" }} />
                        </Form.Item>
                      )
                    },
                    {
                      title: "操作",
                      render: (_, field, index) => (
                        <Button danger type="link" onClick={() => remove(field.name)}>
                          删除
                        </Button>
                      )
                    }
                  ]}
                />
                <Button
                  style={{ marginTop: 8 }}
                  type="dashed"
                  onClick={() => add({ unit: "EA", plan_quantity: 0, unit_price: 0 })}
                  block
                >
                  添加明细行
                </Button>
              </>
            )}
          </Form.List>
        </Form>
      </Modal>
    </Card>
  );
}


