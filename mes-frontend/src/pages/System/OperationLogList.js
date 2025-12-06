import React, { useEffect, useState } from "react";
import { Card, Table, Tag, Space, Input, Button } from "antd";
import { fetchOperationLogs } from "../../api/systemApi";

const successMap = {
  1: { text: "成功", color: "green" },
  0: { text: "失败", color: "red" }
};

export default function OperationLogList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [username, setUsername] = useState("");
  const [module, setModule] = useState("");

  const load = async (params = {}) => {
    setLoading(true);
    try {
      const res = await fetchOperationLogs({
        page: 0,
        page_size: 50,
        username: username || undefined,
        module: module || undefined,
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
          <Button type="primary" onClick={() => load()}>
            查询
          </Button>
        </Space>
      }
    >
      <Table rowKey="id" loading={loading} dataSource={data} columns={columns} />
    </Card>
  );
}



