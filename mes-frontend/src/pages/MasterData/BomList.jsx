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
  fetchBoms,
  createBom,
  updateBom,
  deleteBom,
  fetchMaterials
} from "../../api/masterDataApi";

const statusMap = {
  0: { text: "停用", color: "red" },
  1: { text: "启用", color: "green" }
};

export default function BomList() {
  const [data, setData] = useState([]);
  const [materials, setMaterials] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchBoms({ page: 0, page_size: 50 });
      setData(res.items || []);
    } finally {
      setLoading(false);
    }
  };

  const loadMaterials = async () => {
    const res = await fetchMaterials({ page: 0, page_size: 100 });
    setMaterials(res.items || []);
  };

  useEffect(() => {
    load();
    loadMaterials();
  }, []);

  const openModal = (record) => {
    setCurrent(record || null);
    setModalOpen(true);
    form.resetFields();
    if (record) {
      form.setFieldsValue({
        ...record,
        itemsJson: JSON.stringify(record.items || [], null, 2)
      });
    } else {
      form.setFieldsValue({
        bom_type: 1,
        status: 1,
        is_default: 0,
        itemsJson: "[\n  { \"child_material_id\": 0, \"qty\": 1, \"unit\": \"PCS\" }\n]"
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    let itemsParsed = [];
    try {
      itemsParsed = JSON.parse(values.itemsJson || "[]");
    } catch (error) {
      message.error("BOM明细需为合法 JSON");
      return;
    }
    const payload = {
      material_id: values.material_id,
      bom_code: values.bom_code,
      version: values.version,
      bom_type: values.bom_type,
      is_default: values.is_default,
      status: values.status,
      items: itemsParsed,
      remark: values.remark
    };
    try {
      if (current) {
        await updateBom(current.id, payload);
        message.success("更新成功");
      } else {
        await createBom(payload);
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
      await deleteBom(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "BOM编码", dataIndex: "bom_code" },
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
      title: "组件数量",
      dataIndex: "items",
      render: (items) => (Array.isArray(items) ? items.length : 0)
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
      title="BOM 管理"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建 BOM
        </Button>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />

      <Modal
        open={modalOpen}
        title={current ? "编辑 BOM" : "新建 BOM"}
        width={720}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="material_id" label="父件物料" rules={[{ required: true }]}>
            <Select
              showSearch
              optionFilterProp="children"
              placeholder="选择父件物料"
            >
              {materials.map((m) => (
                <Select.Option key={m.id} value={m.id}>
                  {m.material_name}（{m.material_code}）
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="bom_code" label="BOM编码" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="version" label="版本" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="bom_type" label="BOM类型" rules={[{ required: true }]}>
            <Select>
              <Select.Option value={1}>生产 BOM</Select.Option>
              <Select.Option value={2}>工程 BOM</Select.Option>
              <Select.Option value={3}>维修 BOM</Select.Option>
            </Select>
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
            name="itemsJson"
            label="BOM 明细（JSON）"
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



