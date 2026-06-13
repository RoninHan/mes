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
  fetchLocations,
  createLocation,
  updateLocation,
  deleteLocation,
  fetchWarehouses
} from "../../api/masterDataApi";

const statusMap = {
  0: { text: "停用", color: "red" },
  1: { text: "启用", color: "green" }
};

const locationTypeOptions = [
  { value: 1, label: "普通" },
  { value: 2, label: "冷藏" },
  { value: 3, label: "不合格" },
  { value: 4, label: "其他" }
];

export default function LocationList() {
  const [data, setData] = useState([]);
  const [warehouses, setWarehouses] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchLocations({ page: 0, page_size: 50 });
      setData(res.items || []);
    } finally {
      setLoading(false);
    }
  };

  const loadWarehouses = async () => {
    const res = await fetchWarehouses({ page: 0, page_size: 100 });
    setWarehouses(res.items || []);
  };

  useEffect(() => {
    load();
    loadWarehouses();
  }, []);

  const openModal = (record) => {
    setCurrent(record || null);
    setModalOpen(true);
    form.resetFields();
    if (record) {
      form.setFieldsValue(record);
    } else {
      form.setFieldsValue({
        location_type: 1,
        status: 1
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    try {
      if (current) {
        await updateLocation(current.id, values);
        message.success("更新成功");
      } else {
        await createLocation(values);
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
      await deleteLocation(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "库位编码", dataIndex: "location_code" },
    { title: "库位名称", dataIndex: "location_name" },
    { title: "仓库ID", dataIndex: "warehouse_id" },
    {
      title: "类型",
      dataIndex: "location_type",
      render: (v) => locationTypeOptions.find((item) => item.value === v)?.label || v
    },
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
      title="库位管理"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建库位
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />

      <Modal
        open={modalOpen}
        title={current ? "编辑库位" : "新建库位"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="warehouse_id" label="所属仓库" rules={[{ required: true }]}>
            <Select placeholder="选择仓库">
              {warehouses.map((w) => (
                <Select.Option key={w.id} value={w.id}>
                  {w.warehouse_name}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="location_code" label="库位编码" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="location_name" label="库位名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="location_type" label="库位类型" rules={[{ required: true }]}>
            <Select options={locationTypeOptions} />
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



