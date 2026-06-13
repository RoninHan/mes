import React, { useEffect, useState } from "react";
import { Card, Table, Tag, Space, Input, Button } from "antd";
import { fetchInventory } from "../../api/warehouseApi";

const qualityMap = {
  1: { text: "合格", color: "green" },
  2: { text: "待检", color: "gold" },
  3: { text: "不合格", color: "red" },
  4: { text: "冻结", color: "purple" }
};

export default function InventoryList() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [materialId, setMaterialId] = useState("");
  const [warehouseId, setWarehouseId] = useState("");

  const load = async (params = {}) => {
    setLoading(true);
    try {
      const res = await fetchInventory({ page: 0, page_size: 100, ...params });
      setData(res.items || []);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
  }, []);

  const columns = [
    { title: "库存ID", dataIndex: "id" },
    { title: "物料ID", dataIndex: "material_id" },
    { title: "仓库ID", dataIndex: "warehouse_id" },
    { title: "库位ID", dataIndex: "location_id" },
    { title: "批次号", dataIndex: "batch_no" },
    { title: "序列号", dataIndex: "serial_no" },
    {
      title: "库存数量",
      dataIndex: "quantity",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    {
      title: "可用数量",
      dataIndex: "available_quantity",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    {
      title: "锁定数量",
      dataIndex: "locked_quantity",
      render: (v) => v?.toFixed?.(2) ?? v
    },
    { title: "单位", dataIndex: "unit" }
  ];

  return (
    <Card
      title="库存总览"
      extra={
        <Space>
          <Input
            placeholder="物料ID"
            value={materialId}
            onChange={(e) => setMaterialId(e.target.value)}
            style={{ width: 120 }}
          />
          <Input
            placeholder="仓库ID"
            value={warehouseId}
            onChange={(e) => setWarehouseId(e.target.value)}
            style={{ width: 120 }}
          />
          <Button
            type="primary"
            onClick={() =>
              load({
                material_id: materialId || undefined,
                warehouse_id: warehouseId || undefined
              })
            }
          >
            查询
          </Button>
        </Space>
      }
    >
      <Table rowKey="id" loading={loading} columns={columns} dataSource={data} />
    </Card>
  );
}


