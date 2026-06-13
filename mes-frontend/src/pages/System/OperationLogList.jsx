import React, { useEffect, useState } from "react";
import { Card, Table, Tag, Space, Input, Button } from "antd";
import { fetchOperationLogs } from "../../api/systemApi";

const successMap = {
  1: { text: "成功", color: "green" },
  0: { text: "失败", color: "red" }
};

const DEFAULT_MODULE = "equipment";
const DEFAULT_ACTION = "control_command";

export default function OperationLogList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [username, setUsername] = useState("");
  const [module, setModule] = useState(DEFAULT_MODULE);
  const [action, setAction] = useState(DEFAULT_ACTION);

  const load = async (params = {}) => {
    setLoading(true);
    try {
      const res = await fetchOperationLogs({
        page: 0,
        page_size: 50,
        username: username || undefined,
        module: module || undefined,
        action: action || undefined,
        ...params
      });
      setData(res.items || []);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
  }, []);

  const columns = [
    { title: "用户ID", dataIndex: "user_id" },
    { title: "用户名", dataIndex: "username" },
    { title: "模块", dataIndex: "module" },
    { title: "操作", dataIndex: "action" },
    {
      title: "设备ID",
      dataIndex: "payload",
      render: (payload) => payload?.equipment_id ?? "-"
    },
    {
      title: "命令",
      dataIndex: "payload",
      render: (payload) => payload?.command ?? "-"
    },
    {
      title: "Topic",
      dataIndex: "payload",
      render: (payload) => payload?.topic ?? "-"
    },
    { title: "请求路径", dataIndex: "request_path" },
    { title: "方法", dataIndex: "method" },
    { title: "时间", dataIndex: "request_time" },
    {
      title: "结果",
      dataIndex: "success",
      render: (v) => (
        <Tag color={successMap[v]?.color}>{successMap[v]?.text || "未知"}</Tag>
      )
    },
    { title: "客户端 IP", dataIndex: "client_ip" },
    { title: "错误信息", dataIndex: "error_message", ellipsis: true }
  ];

  return (
    <Card
      title="操作审计日志"
      extra={
        <Space>
          <Input
            placeholder="用户名"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            style={{ width: 140 }}
          />
          <Input
            placeholder="模块"
            value={module}
            onChange={(e) => setModule(e.target.value)}
            style={{ width: 140 }}
          />
          <Input
            placeholder="操作"
            value={action}
            onChange={(e) => setAction(e.target.value)}
            style={{ width: 160 }}
          />
          <Button type="primary" onClick={() => load()}>
            查询
          </Button>
          <Button
            onClick={() => {
              setUsername("");
              setModule("");
              setAction("");
              load({ username: undefined, module: undefined, action: undefined });
            }}
          >
            清空筛选
          </Button>
          <Button
            onClick={() => {
              setModule(DEFAULT_MODULE);
              setAction(DEFAULT_ACTION);
              load({ module: DEFAULT_MODULE, action: DEFAULT_ACTION });
            }}
          >
            控制日志
          </Button>
        </Space>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />
    </Card>
  );
}




