import React, { useEffect, useState } from "react";
import { Card, Col, DatePicker, Row, Select, Space, Statistic } from "antd";
import { Line } from "react-chartjs-2";
import dayjs from "dayjs";
import { fetchEquipmentKpi } from "../../api/equipmentApi";

export default function EquipmentKpiDashboard() {
  const [data, setData] = useState([]);
  const [equipmentId, setEquipmentId] = useState(null);

  const load = async () => {
    const res = await fetchEquipmentKpi({
      equipment_id: equipmentId || undefined,
      page: 0,
      page_size: 100
    });
    setData(res.items || []);
  };

  useEffect(() => {
    load();
  }, [equipmentId]);

  const latest = data[0] || {};

  const chartData = {
    labels: (data || []).map((d) => d.stat_date),
    datasets: [
      {
        label: "OEE",
        data: (data || []).map((d) => d.oee || 0),
        borderColor: "#52c41a"
      }
    ]
  };

  return (
    <Row gutter={16}>
      <Col span={24}>
        <Card
          title="设备KPI看板"
          extra={
            <Space>
              <Select
                placeholder="设备ID"
                style={{ width: 160 }}
                allowClear
                value={equipmentId}
                onChange={setEquipmentId}
              />
              <DatePicker.RangePicker defaultValue={[dayjs().add(-7, "day"), dayjs()]} />
            </Space>
          }
        >
          <Row gutter={16}>
            <Col span={6}>
              <Statistic title="OEE" value={latest.oee || 0} precision={2} suffix="%" />
            </Col>
            <Col span={6}>
              <Statistic title="MTBF(分钟)" value={latest.mtbf_minutes || 0} />
            </Col>
            <Col span={6}>
              <Statistic title="MTTR(分钟)" value={latest.mttr_minutes || 0} />
            </Col>
            <Col span={6}>
              <Statistic title="故障次数" value={latest.fault_count || 0} />
            </Col>
          </Row>
          <div style={{ marginTop: 24 }}>
            <Line data={chartData} />
          </div>
        </Card>
      </Col>
    </Row>
  );
}



