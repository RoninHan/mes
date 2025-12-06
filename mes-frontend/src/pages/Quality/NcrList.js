import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message } from "antd";
import { fetchNcr, createNcr, updateNcr, deleteNcr } from "../../api/qualityApi";

const ncrStatusMap = {
  1: { text: "待处置", color: "default" },
  2: { text: "处置中", color: "processing" },
  3: { text: "已处置", color: "success" },
  4: { text: "已验证", color: "blue" },
  5: { text: "已关闭", color: "green" }
};

const defectLevelMap = {
  1: { text: "致命", color: "red" },
  2: { text: "严重", color: "orange" },
  3: { text: "一般", color: "yellow" },
  4: { text: "轻微", color: "green" }
};

export default function NcrList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchNcr({ page: 0, page_size: 50 });
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
      form.setFieldsValue({
        ...record,
        found_date: record.found_date
      });
    } else {
      form.setFieldsValue({
        source_type: 1,
        defect_level: 3,
        ncr_status: 1
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      defect_quantity: parseFloat(values.defect_quantity) || 0,
      found_date: values.found_date,
      found_time: new Date().toISOString()
    };
    try {
      if (current) {
        await updateNcr(current.id, payload);
        message.success("更新成功");
      } else {
        await createNcr(payload);
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
      await deleteNcr(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "NCR编号", dataIndex: "ncr_no", width: 150 },
    { title: "物料ID", dataIndex: "material_id", width: 100 },
    { title: "批次号", dataIndex: "batch_no", width: 120 },
    { title: "不合格数量", dataIndex: "defect_quantity", width: 120 },
    { title: "单位", dataIndex: "unit", width: 80 },
    {
      title: "缺陷等级",
      dataIndex: "defect_level",
      width: 100,
      render: (v) => <Tag color={defectLevelMap[v]?.color}>{defectLevelMap[v]?.text || "未知"}</Tag>
    },
    { title: "发现日期", dataIndex: "found_date", width: 120 },
    {
      title: "NCR状态",
      dataIndex: "ncr_status",
      width: 100,
      render: (v) => <Tag color={ncrStatusMap[v]?.color}>{ncrStatusMap[v]?.text || "未知"}</Tag>
    },
    {
      title: "操作",
      width: 150,
      fixed: "right",
      render: (_, record) => (
        <Space>
          <Button type="link" size="small" onClick={() => openModal(record)}>
            编辑
          </Button>
          <Button type="link" size="small" danger onClick={() => handleDelete(record)}>
            删除
          </Button>
        </Space>
      )
    }
  ];

  return (
    <Card
      title="不合格品记录（NCR）"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建NCR
        </Button>
      }
    >
      <Table
        columns={columns}
        dataSource={data}
        loading={loading}
        rowKey="id"
        scroll={{ x: 1000 }}
        pagination={{ pageSize: 50 }}
      />

      <Modal
        title={current ? "编辑NCR" : "新建NCR"}
        open={modalOpen}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="ncr_no" label="NCR编号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="source_type" label="来源类型" rules={[{ required: true }]}>
            <Select>
              <Select.Option value={1}>来料检验</Select.Option>
              <Select.Option value={2}>过程检验</Select.Option>
              <Select.Option value={3}>成品检验</Select.Option>
              <Select.Option value={4}>客户退货</Select.Option>
              <Select.Option value={5}>其他</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="material_id" label="物料ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="batch_no" label="批次号">
            <Input />
          </Form.Item>
          <Form.Item name="defect_quantity" label="不合格数量" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="unit" label="单位" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="defect_level" label="缺陷等级">
            <Select>
              {Object.entries(defectLevelMap).map(([k, v]) => (
                <Select.Option key={k} value={parseInt(k)}>{v.text}</Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="found_date" label="发现日期" rules={[{ required: true }]}>
            <Input type="date" />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


