import React, { useEffect, useState } from "react";
import { Card, Table, DatePicker, Select, Space } from "antd";
import { fetchStatusLog } from "../../api/equipmentApi";

const { RangePicker } = DatePicker;

export default function EquipmentStatusLog() {
  const [data, setData] = useState([]);
  const [status, setStatus] = useState();

  const load = async (params = {}) => {
    const res = await fetchStatusLog(params);
    setData(res.items || []);
  };

  useEffect(() => {
    load();
  }, []);

  const columns = [
    { title: "设备ID", dataIndex: "equipment_id" },
    { title: "状态", dataIndex: "status" },
    { title: "故障码", dataIndex: "error_code" },
    { title: "故障描述", dataIndex: "error_desc" },
    { title: "时间", dataIndex: "log_time" }
  ];

  return (
    <Card
      title="设备状态日志"
      extra={
        <Space>
          <RangePicker
            onChange={(vals) =>
              load({
                start_time: vals?.[0]?.toISOString(),
                end_time: vals?.[1]?.toISOString(),
                status
              })
            }
          />
          <Select
            allowClear
            style={{ width: 140 }}
            placeholder="状态"
            value={status}
            onChange={(v) => {
              setStatus(v);
              load({ status: v });
            }}
            options={[
              { value: 0, label: "离线" },
              { value: 1, label: "在线" },
              { value: 2, label: "故障" },
              { value: 3, label: "维护中" }
            ]}
          />
        </Space>
      }
    >
      <Table rowKey="id" columns={columns} dataSource={data} />
    </Card>
  );
}


