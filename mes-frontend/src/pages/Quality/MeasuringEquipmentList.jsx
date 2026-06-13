import React, { useEffect, useState } from "react";
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message } from "antd";
import {
  fetchMeasuringEquipment,
  createMeasuringEquipment,
  updateMeasuringEquipment,
  deleteMeasuringEquipment
} from "../../api/qualityApi";

const equipmentTypeMap = {
  1: { text: "量具", color: "blue" },
  2: { text: "仪器", color: "cyan" },
  3: { text: "检测设备", color: "green" },
  4: { text: "其他", color: "default" }
};

const equipmentStatusMap = {
  1: { text: "正常", color: "success" },
  2: { text: "待校准", color: "warning" },
  3: { text: "校准中", color: "processing" },
  4: { text: "停用", color: "default" },
  5: { text: "报废", color: "error" }
};

export default function MeasuringEquipmentList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchMeasuringEquipment({ page: 0, page_size: 50 });
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
        next_calibration_date: record.next_calibration_date
      });
    } else {
      form.setFieldsValue({
        equipment_type: 1,
        equipment_status: 1,
        calibration_cycle: 365
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      calibration_cycle: parseInt(values.calibration_cycle) || 365,
      next_calibration_date: values.next_calibration_date
    };
    try {
      if (current) {
        await updateMeasuringEquipment(current.id, payload);
        message.success("更新成功");
      } else {
        await createMeasuringEquipment(payload);
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
      await deleteMeasuringEquipment(record.id);
      message.success("已删除");
      load();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const columns = [
    { title: "设备编码", dataIndex: "equipment_code", width: 150 },
    { title: "设备名称", dataIndex: "equipment_name", width: 200 },
    { title: "设备型号", dataIndex: "equipment_model", width: 150 },
    {
      title: "设备类型",
      dataIndex: "equipment_type",
      width: 100,
      render: (v) => <Tag color={equipmentTypeMap[v]?.color}>{equipmentTypeMap[v]?.text || "未知"}</Tag>
    },
    {
      title: "设备状态",
      dataIndex: "equipment_status",
      width: 100,
      render: (v) => <Tag color={equipmentStatusMap[v]?.color}>{equipmentStatusMap[v]?.text || "未知"}</Tag>
    },
    { title: "下次校准日期", dataIndex: "next_calibration_date", width: 150 },
    { title: "存放位置", dataIndex: "location", width: 150 },
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
      title="测量设备管理"
      extra={
        <Button type="primary" onClick={() => openModal(null)}>
          新建设备
        </Button>
      }
    >
      <Table
        columns={columns}
        dataSource={data}
        loading={loading}
        rowKey="id"
        scroll={{ x: 1200 }}
        pagination={{ pageSize: 50 }}
      />

      <Modal
        title={current ? "编辑测量设备" : "新建测量设备"}
        open={modalOpen}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="equipment_code" label="设备编码" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="equipment_name" label="设备名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="equipment_model" label="设备型号">
            <Input />
          </Form.Item>
          <Form.Item name="equipment_type" label="设备类型" rules={[{ required: true }]}>
            <Select>
              {Object.entries(equipmentTypeMap).map(([k, v]) => (
                <Select.Option key={k} value={parseInt(k)}>{v.text}</Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="calibration_cycle" label="校准周期（天）">
            <Input type="number" />
          </Form.Item>
          <Form.Item name="next_calibration_date" label="下次校准日期">
            <Input type="date" />
          </Form.Item>
          <Form.Item name="equipment_status" label="设备状态">
            <Select>
              {Object.entries(equipmentStatusMap).map(([k, v]) => (
                <Select.Option key={k} value={parseInt(k)}>{v.text}</Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="location" label="存放位置">
            <Input />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}


