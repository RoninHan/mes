import React, { useEffect, useState } from "react";
import { Card, Descriptions, Tabs } from "antd";
import { useParams } from "react-router-dom";
import { fetchEquipmentDetail, fetchStatusLog } from "../../api/equipmentApi";
import { useMqtt } from "../../mqtt/mqttHook";

export default function EquipmentDetail() {
  const { id } = useParams();
  const [detail, setDetail] = useState(null);
  const [logs, setLogs] = useState([]);

  useEffect(() => {
    fetchEquipmentDetail(id).then(setDetail);
    fetchStatusLog({ equipment_id: id }).then((res) => setLogs(res.items || []));
  }, [id]);

  const topic = detail?.mqtt_topic || `equipment/${id}/status`;
  useMqtt(Number(id), topic);

  return (
    <Card title={`设备详情 #${id}`}>
      <Tabs
        items={[
          {
            key: "base",
            label: "基本信息",
            children: (
              <Descriptions bordered column={2}>
                <Descriptions.Item label="设备编码">{detail?.equipment_code}</Descriptions.Item>
                <Descriptions.Item label="设备名称">{detail?.equipment_name}</Descriptions.Item>
                <Descriptions.Item label="类型">{detail?.equipment_type}</Descriptions.Item>
                <Descriptions.Item label="型号">{detail?.model}</Descriptions.Item>
                <Descriptions.Item label="位置">{detail?.location}</Descriptions.Item>
                <Descriptions.Item label="负责人">{detail?.responsible_person}</Descriptions.Item>
              </Descriptions>
            )
          },
          {
            key: "status",
            label: "实时状态",
            children: <div>实时状态与运行参数（接入 MQTT 数据后自动刷新）</div>
          },
          {
            key: "log",
            label: "状态日志",
            children: (
              <pre style={{ maxHeight: 300, overflow: "auto" }}>
                {JSON.stringify(logs, null, 2)}
              </pre>
            )
          }
        ]}
      />
    </Card>
  );
}


