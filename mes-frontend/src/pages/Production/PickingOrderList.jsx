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
  Tag,
  message
} from "antd";
import dayjs from "dayjs";
import {
  fetchPickingOrders,
  fetchPickingOrder,
  createPickingOrder,
  updatePickingOrder,
  deletePickingOrder
} from "../../api/productionApi";

const statusMap = {
  1: { text: "待领料", color: "default" },
  2: { text: "部分领料", color: "gold" },
  3: { text: "已完成", color: "green" },
  4: { text: "已取消", color: "red" }
};

export default function PickingOrderList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [currentId, setCurrentId] = useState(null);
  const [form] = Form.useForm();
  const [detailForm] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchPickingOrders({ page: 0, page_size: 50 });
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
      details: [
        {
          material_id: null,
          warehouse_id: null,
          plan_quantity: 0,
          unit: "EA"
        }
      ]
    });
    form.setFieldsValue({
      picking_type: 1,
      plan_picking_date: dayjs()
    });
    setModalOpen(true);
  };

  const openEdit = async (record) => {
    const full = await fetchPickingOrder(record.id);
    setCurrentId(record.id);
    form.setFieldsValue({
      picking_no: full.header.picking_no,
      production_order_id: full.header.production_order_id,
      warehouse_id: full.header.warehouse_id,
      work_order_id: full.header.work_order_id,
      picking_type: full.header.picking_type,
      plan_picking_date: full.header.plan_picking_date ? dayjs(full.header.plan_picking_date) : null,
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
      plan_picking_date: header.plan_picking_date?.format("YYYY-MM-DD") || null,
      details
    };
    try {
      if (currentId) {
        await updatePickingOrder(currentId, payload);
        message.success("更新成功");
      } else {
        await createPickingOrder(payload);
        message.success("创建成功");
      }
      setModalOpen(false);
      load();
    } catch {
      message.error("保存失败");
    }
  };

  const handleDelete = async (record) => {
    await deletePickingOrder(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "领料单号", dataIndex: "picking_no" },
    { title: "生产订单ID", dataIndex: "production_order_id" },
    { title: "仓库ID", dataIndex: "warehouse_id" },
    { title: "工单ID", dataIndex: "work_order_id" },
    { title: "计划日期", dataIndex: "plan_picking_date" },
    { title: "实际日期", dataIndex: "actual_picking_date" },
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
      title="领料单"
      extra={
        <Button type="primary" onClick={openNew}>
          新建领料单
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />

      <Modal
        open={modalOpen}
        title={currentId ? "编辑领料单" : "新建领料单"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={900}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="picking_no" label="领料单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item
            name="production_order_id"
            label="生产订单ID"
            rules={[{ required: true }]}
          >
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="warehouse_id" label="仓库ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="work_order_id" label="工单ID">
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="picking_type" label="领料类型" rules={[{ required: true }]}>
            <InputNumber min={1} max={3} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="plan_picking_date" label="计划领料日期">
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
                      render: (_, field) => (
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


