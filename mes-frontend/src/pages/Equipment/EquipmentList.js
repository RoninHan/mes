import React, { useEffect, useState } from "react";
import { Card, Table, Tag, Button, Space, Select } from "antd";
import { useNavigate } from "react-router-dom";
import { fetchEquipmentList } from "../../api/equipmentApi";

const statusColors = {
  0: "default",
  1: "green",
  2: "red",
  3: "orange"
};

const statusText = {
  0: "离线",
  1: "在线",
  2: "故障",
  3: "维护中"
};

export default function EquipmentList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState();
  const navigate = useNavigate();

  const load = async (params = {}) => {
    setLoading(true);
    try {
      const res = await fetchEquipmentList(params);
      setData(res.items || []);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
  }, []);

  const columns = [
    { title: "设备编码", dataIndex: "equipment_code" },
    { title: "设备名称", dataIndex: "equipment_name" },
    { title: "类型", dataIndex: "equipment_type" },
    {
      title: "状态",
      dataIndex: "status",
      render: (v) => <Tag color={statusColors[v] || "default"}>{statusText[v] || "未知"}</Tag>
    },
    { title: "安装位置", dataIndex: "location" },
    {
      title: "操作",
      render: (_, record) => (
        <Space>
          <Button type="link" onClick={() => navigate(`/equipment/${record.id}`)}>
            详情
          </Button>
          <Button type="link" onClick={() => navigate(`/equipment/${record.id}/edit`)}>
            编辑
          </Button>
        </Space>
      )
    }
  ];

  return (
    <Card
      title="设备台账"
      extra={
        <Space>
          <Select
            allowClear
            style={{ width: 140 }}
            placeholder="按状态筛选"
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
          <Button type="primary" onClick={() => navigate("/equipment/new")}>
            新增设备
          </Button>
        </Space>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
    </Card>
  );
}


