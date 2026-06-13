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
  fetchWorkshops,
  createWorkshop,
  updateWorkshop,
  deleteWorkshop
} from "../../api/masterDataApi";

const statusMap = {
  0: { text: "停用", color: "red" },
  1: { text: "启用", color: "green" }
};

const workshopTypeOptions = [
  { value: 1, label: "制造" },
  { value: 2, label: "装配" },
  { value: 3, label: "包装" },
  { value: 4, label: "其他" }
];

export default function WorkshopList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchWorkshops({ page: 0, page_size: 50 });
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
        workshop_type: 1,
        status: 1
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    try {
      if (current) {
        await updateWorkshop(current.id, values);
        message.success("更新成功");
      } else {
        await createWorkshop(values);
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
      await deleteWorkshop(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "车间编码", dataIndex: "workshop_code" },
    { title: "车间名称", dataIndex: "workshop_name" },
    {
      title: "类型",
      dataIndex: "workshop_type",
      render: (v) => workshopTypeOptions.find((item) => item.value === v)?.label || v
    },
    { title: "负责人ID", dataIndex: "manager_id" },
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
      title="车间管理"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建车间
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />

      <Modal
        open={modalOpen}
        title={current ? "编辑车间" : "新建车间"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="workshop_code" label="车间编码" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="workshop_name" label="车间名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="workshop_type" label="车间类型" rules={[{ required: true }]}>
            <Select options={workshopTypeOptions} />
          </Form.Item>
          <Form.Item name="manager_id" label="负责人ID">
            <Input type="number" />
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



