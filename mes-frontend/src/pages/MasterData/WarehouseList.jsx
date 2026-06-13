import React, { useEffect, useState } from "react";
import {
  Button,
  Card,
  Form,
  Input,
  Modal,
  Select,
  Space,
  Table,
  Tag,
  message
} from "antd";
import {
  fetchWarehouses,
  createWarehouse,
  updateWarehouse,
  deleteWarehouse
} from "../../api/masterDataApi";

const statusMap = {
  0: { text: "停用", color: "red" },
  1: { text: "启用", color: "green" }
};

const warehouseTypeOptions = [
  { value: 1, label: "原材料" },
  { value: 2, label: "半成品" },
  { value: 3, label: "成品" },
  { value: 4, label: "退货/不良品" },
  { value: 5, label: "其他" }
];

export default function WarehouseList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchWarehouses({ page: 0, page_size: 50 });
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
      form.setFieldsValue(record);
    } else {
      form.setFieldsValue({
        warehouse_type: 1,
        status: 1
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    try {
      if (current) {
        await updateWarehouse(current.id, values);
        message.success("更新成功");
      } else {
        await createWarehouse(values);
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
      await deleteWarehouse(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "仓库编码", dataIndex: "warehouse_code" },
    { title: "仓库名称", dataIndex: "warehouse_name" },
    {
      title: "类型",
      dataIndex: "warehouse_type",
      render: (v) => warehouseTypeOptions.find((item) => item.value === v)?.label || v
    },
    { title: "位置", dataIndex: "location" },
    {
      title: "状态",
      dataIndex: "status",
      render: (v) => <Tag color={statusMap[v]?.color}>{statusMap[v]?.text || "未知"}</Tag>
    },
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
      title="仓库管理"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建仓库
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />

      <Modal
        open={modalOpen}
        title={current ? "编辑仓库" : "新建仓库"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="warehouse_code" label="仓库编码" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="warehouse_name" label="仓库名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="warehouse_type" label="仓库类型" rules={[{ required: true }]}>
            <Select options={warehouseTypeOptions} />
          </Form.Item>
          <Form.Item name="location" label="位置">
            <Input />
          </Form.Item>
          <Form.Item name="status" label="状态" rules={[{ required: true }]}>
            <Select>
              <Select.Option value={1}>启用</Select.Option>
              <Select.Option value={0}>停用</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}



