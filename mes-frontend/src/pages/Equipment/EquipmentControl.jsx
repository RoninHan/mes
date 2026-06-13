import React, { useState } from "react";
import { Card, Button, Input, Space, message } from "antd";
import { sendControlCommand } from "../../api/equipmentApi";

export default function EquipmentControl() {
  const [equipmentId, setEquipmentId] = useState("");
  const [param, setParam] = useState("");

  const send = async (command) => {
    if (!equipmentId) {
      message.warning("请先输入设备ID");
      return;
    }
    let parsedParam = param;
    if (typeof param === "string" && param.trim()) {
      try {
        parsedParam = JSON.parse(param);
      } catch {
        // 保持原样字符串，后端会按原值透传
      }
    }

    try {
      const res = await sendControlCommand(equipmentId, { command, param: parsedParam });
      if (res?.delivered) {
        message.success(res?.message || "指令已发送");
      } else {
        message.warning(res?.message || "命令未实际下发");
      }
    } catch (err) {
      const msg = err?.response?.data?.message || err?.message || "发送失败";
      message.error(msg);
    }
  };

  return (
    <Card title="设备控制">
      <Space direction="vertical" style={{ width: 400 }}>
        <Input
          placeholder="设备ID"
          value={equipmentId}
          onChange={(e) => setEquipmentId(e.target.value)}
        />
        <Input
          placeholder="参数（如转速等，JSON字符串）"
          value={param}
          onChange={(e) => setParam(e.target.value)}
        />
        <Space>
          <Button type="primary" onClick={() => send("start")}>
            启动
          </Button>
          <Button danger onClick={() => send("stop")}>
            停止
          </Button>
        </Space>
      </Space>
    </Card>
  );
}


