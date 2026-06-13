import React, { useEffect, useState } from "react";
import { Alert, Button, Card, Form, Input, Modal, Select, Space, Table, Tag, message, DatePicker, InputNumber } from "antd";
import { LinkOutlined } from "@ant-design/icons";
import dayjs from "dayjs";
import {
  fetchProductionOrders,
  createProductionOrder,
  updateProductionOrder,
  deleteProductionOrder
} from "../../api/productionApi";

const ERP_URL = import.meta.env.VITE_ERP_URL || "http://localhost:3000";

const statusMap = {
  1: { text: "待发布", color: "default" },
  2: { text: "已发布", color: "cyan" },
  3: { text: "已下达", color: "blue" },
  4: { text: "生产中", color: "gold" },
  5: { text: "已完工", color: "green" },
  6: { text: "已入库", color: "purple" },
  7: { text: "已取消", color: "red" }
};

export default function ProductionOrderList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [current, setCurrent] = useState(null);
  const [form] = Form.useForm();
  const [searchKeyword, setSearchKeyword] = useState("");

  // 读取 URL 参数，支持从 ERP 跳转过来时自动过滤
  const urlParams = new URLSearchParams(window.location.search);
  const erpOrderNo = urlParams.get("erp_order_no") || "";
  const [filterKeyword, setFilterKeyword] = useState(erpOrderNo);

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchProductionOrders({ page: 0, page_size: 100 });
      setData(res.items || []);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
    // 若有 erp_order_no 参数则展示来源提示并清除 URL（避免刷新重复触发）
    if (erpOrderNo) {
      window.history.replaceState({}, "", window.location.pathname);
    }
  }, []);

  // 前端过滤（按订单号/物料ID等关键字）
  const filteredData = filterKeyword
    ? data.filter(
        (row) =>
          String(row.order_no || "").toLowerCase().includes(filterKeyword.toLowerCase()) ||
          String(row.material_id || "").includes(filterKeyword)
      )
    : data;

  const openModal = (record) => {
    setCurrent(record || null);
    setModalOpen(true);
    form.resetFields();
    if (record) {
      form.setFieldsValue({
        ...record,
        plan_start_date: dayjs(record.plan_start_date),
        plan_end_date: dayjs(record.plan_end_date)
      });
    } else {
      form.setFieldsValue({
        order_status: 1,
        plan_start_date: dayjs(),
        plan_end_date: dayjs().add(3, "day")
      });
    }
  };

  const handleOk = async () => {
    const values = await form.validateFields();
    const payload = {
      ...values,
      plan_start_date: values.plan_start_date.format("YYYY-MM-DD"),
      plan_end_date: values.plan_end_date.format("YYYY-MM-DD")
    };
    if (current) {
      await updateProductionOrder(current.id, payload);
      message.success("更新成功");
    } else {
      await createProductionOrder(payload);
      message.success("创建成功");
    }
    setModalOpen(false);
    load();
  };

  const handleDelete = async (record) => {
    await deleteProductionOrder(record.id);
    message.success("已删除");
    load();
  };

  const columns = [
    {
      title: "订单号",
      dataIndex: "order_no",
      render: (v) => <span style={{ fontFamily: "monospace" }}>{v}</span>
    },
    { title: "计划ID", dataIndex: "plan_id" },
    { title: "物料ID", dataIndex: "material_id" },
    {
      title: "计划数量",
      dataIndex: "plan_quantity",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    {
      title: "状态",
      dataIndex: "order_status",
      render: (v) => <Tag color={statusMap[v]?.color}>{statusMap[v]?.text || "未知"}</Tag>
    },
    { title: "开始日期", dataIndex: "plan_start_date" },
    { title: "结束日期", dataIndex: "plan_end_date" },
    {
      title: "操作",
      render: (_, record) => (
        <Space>
          <Button type="link" onClick={() => openModal(record)}>
            编辑
          </Button>
          <Button type="link" danger onClick={() => handleDelete(record)}>
            删除
          </Button>
          {/* ERP 反向跳转：在 ERP 中查看对应工单 */}
          <Button
            type="link"
            size="small"
            icon={<LinkOutlined />}
            title="在 ERP 中查看对应工单"
            onClick={() => {
              const url = new URL("/production", ERP_URL);
              window.open(url.toString(), "_blank", "noopener");
            }}
            style={{ color: "#aaa", fontSize: 12 }}
          >
            ERP
          </Button>
        </Space>
      )
    }
  ];

  return (
    <Card
      title="生产订单"
      extra={
        <Space>
          <Input.Search
            placeholder="订单号 / 物料ID"
            allowClear
            value={filterKeyword}
            onChange={(e) => setFilterKeyword(e.target.value)}
            onSearch={(v) => setFilterKeyword(v)}
            style={{ width: 200 }}
          />
          <Button type="primary" onClick={() => openModal(null)}>
            新建订单
          </Button>
        </Space>
      }
    >
      {/* 来自 ERP 跳转的提示横幅 */}
      {erpOrderNo && (
        <Alert
          type="info"
          showIcon
          style={{ marginBottom: 12 }}
          message={
            <span>
              已从 ERP 跳转，正在显示工单号包含{" "}
              <strong>{erpOrderNo}</strong> 的生产订单。
              <Button
                type="link"
                size="small"
                onClick={() => setFilterKeyword("")}
                style={{ padding: "0 4px" }}
              >
                清除过滤
              </Button>
            </span>
          }
        />
      )}

      <Table rowKey="id" loading={loading} columns={columns} dataSource={filteredData} />
      <Modal
        open={modalOpen}
        title={current ? "编辑生产订单" : "新建生产订单"}
        onOk={handleOk}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form layout="vertical" form={form}>
          <Form.Item name="order_no" label="订单号" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="plan_id" label="计划ID">
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="material_id" label="物料ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="plan_quantity" label="计划数量" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} min={0} />
          </Form.Item>
          <Form.Item name="order_status" label="状态" rules={[{ required: true }]}>
            <Select
              options={Object.entries(statusMap).map(([value, item]) => ({
                value: Number(value),
                label: item.text
              }))}
            />
          </Form.Item>
          <Form.Item name="plan_start_date" label="开始日期" rules={[{ required: true }]}>
            <DatePicker />
          </Form.Item>
          <Form.Item name="plan_end_date" label="结束日期" rules={[{ required: true }]}>
            <DatePicker />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}
