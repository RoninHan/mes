import React, { useEffect, useState } from "react";
import { Button, Card, DatePicker, Form, Input, InputNumber, Modal, Select, Space, Table, Tag, message } from "antd";
import dayjs from "dayjs";
import {
  fetchOutboundOrders,
  fetchOutboundOrder,
  createOutboundOrder,
  updateOutboundOrder,
  deleteOutboundOrder
} from "../../api/warehouseApi";

const statusMap = {
  1: { text: "待出库", color: "default" },
  2: { text: "部分出库", color: "gold" },
  3: { text: "已完成", color: "green" },
  4: { text: "已取消", color: "red" }
};

export default function OutboundOrderList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [currentId, setCurrentId] = useState(null);
  const [form] = Form.useForm();
  const [detailForm] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchOutboundOrders({ page: 0, page_size: 50 });
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
    detailForm.setFieldsValue({
      details: [{ material_id: null, warehouse_id: null, plan_quantity: 0, unit: "EA" }]
    });
    form.setFieldsValue({
      outbound_type: 1,
      plan_outbound_date: dayjs()
    });
    setModalOpen(true);
  };

  const openEdit = async (record) => {
    const full = await fetchOutboundOrder(record.id);
    setCurrentId(record.id);
    form.setFieldsValue({
      outbound_no: full.header.outbound_no,
      outbound_type: full.header.outbound_type,
      warehouse_id: full.header.warehouse_id,
      customer_id: full.header.customer_id,
      plan_outbound_date: full.header.plan_outbound_date ? dayjs(full.header.plan_outbound_date) : null,
      remark: full.header.remark
    });
    detailForm.setFieldsValue({
      details: full.details.map((d) => ({
        material_id: d.material_id,
        warehouse_id: d.warehouse_id,
        location_id: d.location_id,
        batch_no: d.batch_no,
        plan_quantity: d.plan_quantity,
        unit: d.unit
      }))
    });
    setModalOpen(true);
  };

  const handleOk = async () => {
    const header = await form.validateFields();
    const { details } = await detailForm.validateFields();
    const payload = {
      ...header,
      plan_outbound_date: header.plan_outbound_date?.format("YYYY-MM-DD") || null,
      details
    };
    try {
      if (currentId) {
        await updateOutboundOrder(currentId, payload);
        message.success("更新成功");
      } else {
        await createOutboundOrder(payload);
        message.success("创建成功");
      }
      setModalOpen(false);
      load();
    } catch (e) {
      message.error("保存失败");
    }
  };

  const handleDelete = async (record) => {
    await deleteOutboundOrder(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "出库单号", dataIndex: "outbound_no" },
    { title: "类型", dataIndex: "outbound_type" },
    { title: "仓库ID", dataIndex: "warehouse_id" },
    { title: "客户ID", dataIndex: "customer_id" },
    { title: "计划日期", dataIndex: "plan_outbound_date" },
    { title: "实际日期", dataIndex: "actual_outbound_date" },
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
      title="出库单"
      extra={
        <Button type="primary" onClick={openNew}>
          新建出库单
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
      <Modal
        open={modalOpen}
        title={currentId ? "编辑出库单" : "新建出库单"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={900}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="outbound_no" label="出库单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="outbound_type" label="出库类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 1, label: "销售出库" },
                { value: 2, label: "生产领料" },
                { value: 3, label: "调拨出库" },
                { value: 4, label: "退货出库" },
                { value: 5, label: "其他出库" }
              ]}
            />
          </Form.Item>
          <Form.Item name="warehouse_id" label="仓库ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="customer_id" label="客户ID">
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="plan_outbound_date" label="计划出库日期">
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
                      render: (_, field) => (
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
                      title: "仓库ID",
                      render: (_, field) => (
                        <Form.Item
                          name={[field.name, "warehouse_id"]}
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
                      title: "操作",
                      render: (_, field, index) => (
                        <Button type="link" danger onClick={() => remove(field.name)}>
                          删除
                        </Button>
                      )
                    }
                  ]}
                />
                <Button style={{ marginTop: 8 }} onClick={() => add()}>
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


