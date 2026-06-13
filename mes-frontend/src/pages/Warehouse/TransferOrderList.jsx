import React, { useEffect, useState } from "react";
import { Button, Card, DatePicker, Form, Input, InputNumber, Modal, Space, Table, Tag, message } from "antd";
import dayjs from "dayjs";
import {
  fetchTransferOrders,
  fetchTransferOrder,
  createTransferOrder,
  updateTransferOrder,
  deleteTransferOrder
} from "../../api/warehouseApi";

const statusMap = {
  1: { text: "待调拨", color: "default" },
  2: { text: "部分调拨", color: "gold" },
  3: { text: "已完成", color: "green" },
  4: { text: "已取消", color: "red" }
};

export default function TransferOrderList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [currentId, setCurrentId] = useState(null);
  const [form] = Form.useForm();
  const [detailForm] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchTransferOrders({ page: 0, page_size: 50 });
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
          from_warehouse_id: null,
          to_warehouse_id: null,
          plan_quantity: 0,
          unit: "EA"
        }
      ]
    });
    form.setFieldsValue({
      plan_transfer_date: dayjs()
    });
    setModalOpen(true);
  };

  const openEdit = async (record) => {
    const full = await fetchTransferOrder(record.id);
    setCurrentId(record.id);
    form.setFieldsValue({
      transfer_no: full.header.transfer_no,
      from_warehouse_id: full.header.from_warehouse_id,
      to_warehouse_id: full.header.to_warehouse_id,
      plan_transfer_date: full.header.plan_transfer_date ? dayjs(full.header.plan_transfer_date) : null,
      remark: full.header.remark
    });
    detailForm.setFieldsValue({
      details: full.details.map((d) => ({
        material_id: d.material_id,
        from_warehouse_id: d.from_warehouse_id,
        from_location_id: d.from_location_id,
        to_warehouse_id: d.to_warehouse_id,
        to_location_id: d.to_location_id,
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
      plan_transfer_date: header.plan_transfer_date?.format("YYYY-MM-DD") || null,
      details
    };
    try {
      if (currentId) {
        await updateTransferOrder(currentId, payload);
        message.success("更新成功");
      } else {
        await createTransferOrder(payload);
        message.success("创建成功");
      }
      setModalOpen(false);
      load();
    } catch (e) {
      message.error("保存失败");
    }
  };

  const handleDelete = async (record) => {
    await deleteTransferOrder(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    { title: "调拨单号", dataIndex: "transfer_no" },
    { title: "来源仓库", dataIndex: "from_warehouse_id" },
    { title: "目标仓库", dataIndex: "to_warehouse_id" },
    { title: "计划日期", dataIndex: "plan_transfer_date" },
    { title: "实际日期", dataIndex: "actual_transfer_date" },
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
      title="调拨单"
      extra={
        <Button type="primary" onClick={openNew}>
          新建调拨单
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
      <Modal
        open={modalOpen}
        title={currentId ? "编辑调拨单" : "新建调拨单"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={900}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="transfer_no" label="调拨单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="from_warehouse_id" label="来源仓库ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="to_warehouse_id" label="目标仓库ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="plan_transfer_date" label="计划调拨日期">
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
                      title: "来源仓库ID",
                      render: (_, field) => (
                        <Form.Item
                          name={[field.name, "from_warehouse_id"]}
                          rules={[{ required: true }]}
                          style={{ marginBottom: 0 }}
                        >
                          <InputNumber style={{ width: "100%" }} />
                        </Form.Item>
                      )
                    },
                    {
                      title: "来源库位ID",
                      render: (_, field) => (
                        <Form.Item
                          name={[field.name, "from_location_id"]}
                          style={{ marginBottom: 0 }}
                        >
                          <InputNumber style={{ width: "100%" }} />
                        </Form.Item>
                      )
                    },
                    {
                      title: "目标仓库ID",
                      render: (_, field) => (
                        <Form.Item
                          name={[field.name, "to_warehouse_id"]}
                          rules={[{ required: true }]}
                          style={{ marginBottom: 0 }}
                        >
                          <InputNumber style={{ width: "100%" }} />
                        </Form.Item>
                      )
                    },
                    {
                      title: "目标库位ID",
                      render: (_, field) => (
                        <Form.Item
                          name={[field.name, "to_location_id"]}
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


