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
  fetchProcessRoutes,
  createProcessRoute,
  updateProcessRoute,
  deleteProcessRoute,
  fetchMaterials,
  fetchWorkshops
} from "../../api/masterDataApi";

const statusMap = {
  0: { text: "停用", color: "red" },
  1: { text: "启用", color: "green" }
};

export default function ProcessRouteList() {
  const [data, setData] = useState([]);
  const [materials, setMaterials] = useState([]);
  const [workshops, setWorkshops] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchProcessRoutes({ page: 0, page_size: 50 });
      setData(res.items || []);
    } finally {
      setLoading(false);
    }
  };

  const loadMasterData = async () => {
    const [materialsRes, workshopRes] = await Promise.all([
      fetchMaterials({ page: 0, page_size: 100 }),
      fetchWorkshops({ page: 0, page_size: 100 })
    ]);
    setMaterials(materialsRes.items || []);
    setWorkshops(workshopRes.items || []);
  };

  useEffect(() => {
    load();
    loadMasterData();
  }, []);

  const openModal = (record) => {
    setCurrent(record || null);
    setModalOpen(true);
    form.resetFields();
    if (record) {
      form.setFieldsValue({
        ...record,
        operationsJson: JSON.stringify(record.operations || [], null, 2)
      });
    } else {
      form.setFieldsValue({
        is_default: 0,
        status: 1,
        operationsJson:
          '[\n  { "op_no": 10, "op_name": "工序1", "workshop_id": 1, "std_cycle_time_sec": 60 }\n]'
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    let operationsParsed = [];
    try {
      operationsParsed = JSON.parse(values.operationsJson || "[]");
    } catch (error) {
      message.error("工序列表需为合法 JSON");
      return;
    }
    const payload = {
      material_id: values.material_id,
      route_code: values.route_code,
      route_name: values.route_name,
      version: values.version,
      is_default: values.is_default,
      status: values.status,
      operations: operationsParsed,
      remark: values.remark
    };
    try {
      if (current) {
        await updateProcessRoute(current.id, payload);
        message.success("更新成功");
      } else {
        await createProcessRoute(payload);
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
      await deleteProcessRoute(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "路线编码", dataIndex: "route_code" },
    { title: "路线名称", dataIndex: "route_name" },
    { title: "物料ID", dataIndex: "material_id" },
    { title: "版本", dataIndex: "version" },
    {
      title: "默认",
      dataIndex: "is_default",
      render: (v) => (v === 1 ? <Tag color="green">是</Tag> : <Tag>否</Tag>)
    },
    {
      title: "状态",
      dataIndex: "status",
      render: (v) => <Tag color={statusMap[v]?.color}>{statusMap[v]?.text || "未知"}</Tag>
    },
    {
      title: "工序数",
      dataIndex: "operations",
      render: (ops) => (Array.isArray(ops) ? ops.length : 0)
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
      title="工艺路线"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建路线
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />

      <Modal
        open={modalOpen}
        title={current ? "编辑工艺路线" : "新建工艺路线"}
        width={720}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="material_id" label="物料" rules={[{ required: true }]}>
            <Select
              showSearch
              optionFilterProp="children"
              placeholder="选择物料"
            >
              {materials.map((m) => (
                <Select.Option key={m.id} value={m.id}>
                  {m.material_name}（{m.material_code}）
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="route_code" label="路线编码" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="route_name" label="路线名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="version" label="版本" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="is_default" label="默认" rules={[{ required: true }]}>
            <Select>
              <Select.Option value={1}>是</Select.Option>
              <Select.Option value={0}>否</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="status" label="状态" rules={[{ required: true }]}>
            <Select>
              <Select.Option value={1}>启用</Select.Option>
              <Select.Option value={0}>停用</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item
            name="operationsJson"
            label="工序列表（JSON）"
            rules={[{ required: true }]}
          >
            <Input.TextArea rows={8} />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}



