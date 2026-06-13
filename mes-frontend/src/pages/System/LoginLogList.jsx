import React, { useEffect, useState } from "react";
import { Card, Table, Tag, Space, Input, Button } from "antd";
import { fetchLoginLogs } from "../../api/systemApi";

const resultMap = {
  1: { text: "成功", color: "green" },
  2: { text: "失败", color: "red" }
};

export default function LoginLogList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [username, setUsername] = useState("");

  const load = async (params = {}) => {
    setLoading(true);
    try {
      const res = await fetchLoginLogs({
        page: 0,
        page_size: 50,
        username: username || undefined,
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
    { title: "登录时间", dataIndex: "login_time" },
    { title: "IP", dataIndex: "login_ip" },
    { title: "User-Agent", dataIndex: "user_agent", ellipsis: true },
    {
      title: "结果",
      dataIndex: "result",
      render: (v) => (
        <Tag color={resultMap[v]?.color}>{resultMap[v]?.text || "未知"}</Tag>
      )
    },
    { title: "失败原因", dataIndex: "fail_reason" }
  ];

  return (
    <Card
      title="登录日志"
      extra={
        <Space>
          <Input
            placeholder="用户名"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            style={{ width: 160 }}
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



