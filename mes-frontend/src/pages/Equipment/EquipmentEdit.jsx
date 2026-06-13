import React, { useEffect } from "react";
import { Card, Form, Input, DatePicker, Select, Button, Space } from "antd";
import { useNavigate, useParams } from "react-router-dom";
import { createEquipment, fetchEquipmentDetail, saveMqttConfig, updateEquipment, fetchMqttConfig } from "../../api/equipmentApi";

export default function EquipmentEdit() {
  const [form] = Form.useForm();
  const [mqttForm] = Form.useForm();
  const { id } = useParams();
  const navigate = useNavigate();
  const isEdit = !!id;

  useEffect(() => {
    if (isEdit) {
      fetchEquipmentDetail(id).then((data) => {
        form.setFieldsValue(data);
      });
      fetchMqttConfig(id).then((data) => {
        mqttForm.setFieldsValue(data);
      });
    }
  }, [id, isEdit, form, mqttForm]);

  const onFinish = async (values) => {
    if (isEdit) {
      await updateEquipment(id, values);
    } else {
      await createEquipment(values);
    }
    navigate("/equipment/list");
  };

  const onSaveMqtt = async (values) => {
    await saveMqttConfig(id, values);
  };

  return (
    <Card title={isEdit ? "编辑设备" : "新增设备"}>
      <Form layout="vertical" form={form} onFinish={onFinish}>
        <Form.Item name="equipment_code" label="设备编码" rules={[{ required: true }]}>
          <Input />
        </Form.Item>
        <Form.Item name="equipment_name" label="设备名称" rules={[{ required: true }]}>
          <Input />
        </Form.Item>
        <Form.Item name="equipment_type" label="设备类型" rules={[{ required: true }]}>
          <Input />
        </Form.Item>
        <Form.Item name="model" label="设备型号">
          <Input />
        </Form.Item>
        <Form.Item name="factory" label="生产厂家">
          <Input />
        </Form.Item>
        <Form.Item name="production_date" label="生产日期">
          <DatePicker />
        </Form.Item>
        <Form.Item name="install_date" label="安装日期">
          <DatePicker />
        </Form.Item>
        <Form.Item name="status" label="状态">
          <Select
            options={[
              { value: 0, label: "离线" },
              { value: 1, label: "在线" },
              { value: 2, label: "故障" },
              { value: 3, label: "维护中" }
            ]}
          />
        </Form.Item>
        <Form.Item name="ip_address" label="设备 IP">
          <Input />
        </Form.Item>
        <Form.Item name="mqtt_topic" label="MQTT 主题" rules={[{ required: true }]}>
          <Input placeholder="equipment/{id}/status" />
        </Form.Item>
        <Form.Item name="location" label="安装位置">
          <Input />
        </Form.Item>
        <Form.Item name="responsible_person" label="负责人">
          <Input />
        </Form.Item>
        <Form.Item name="remark" label="备注">
          <Input.TextArea rows={3} />
        </Form.Item>
        <Form.Item>
          <Space>
            <Button type="primary" htmlType="submit">
              保存
            </Button>
            <Button onClick={() => navigate(-1)}>返回</Button>
          </Space>
        </Form.Item>
      </Form>

      {isEdit && (
        <Card title="MQTT 配置" style={{ marginTop: 24 }}>
          <Form layout="vertical" form={mqttForm} onFinish={onSaveMqtt}>
            <Form.Item name="broker_address" label="Broker 地址" rules={[{ required: true }]}>
              <Input placeholder="tcp://127.0.0.1:1883" />
            </Form.Item>
            <Form.Item name="client_id" label="客户端 ID" rules={[{ required: true }]}>
              <Input />
            </Form.Item>
            <Form.Item name="username" label="用户名">
              <Input />
            </Form.Item>
            <Form.Item name="password" label="密码">
              <Input.Password />
            </Form.Item>
            <Form.Item name="keep_alive" label="心跳间隔(秒)">
              <Input />
            </Form.Item>
            <Form.Item name="qos" label="QoS">
              <Select
                options={[
                  { value: 0, label: "0" },
                  { value: 1, label: "1" },
                  { value: 2, label: "2" }
                ]}
              />
            </Form.Item>
            <Form.Item>
              <Button type="primary" htmlType="submit">
                保存 MQTT 配置
              </Button>
            </Form.Item>
          </Form>
        </Card>
      )}
    </Card>
  );
}


