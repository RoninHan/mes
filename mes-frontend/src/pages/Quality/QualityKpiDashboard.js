import React, { useEffect, useState } from "react";
import { Card, Row, Col, Statistic, Table, Tag, DatePicker, Select, Space, message } from "antd";
import { fetchQualityKpi } from "../../api/qualityApi";
import dayjs from "dayjs";
import { Line, Bar } from "react-chartjs-2";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend
} from "chart.js";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend
);

const kpiTypeMap = {
  1: { text: "日度KPI", color: "blue" },
  2: { text: "周度KPI", color: "cyan" },
  3: { text: "月度KPI", color: "green" },
  4: { text: "年度KPI", color: "orange" }
};

export default function QualityKpiDashboard() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [filters, setFilters] = useState({
    start_date: dayjs().subtract(30, "day").format("YYYY-MM-DD"),
    end_date: dayjs().format("YYYY-MM-DD")
  });
  const [summary, setSummary] = useState({
    avgQualifiedRate: 0,
    avgFirstPassYield: 0,
    totalNcr: 0,
    totalQualityCost: 0
  });

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchQualityKpi({
        ...filters,
        page: 0,
        page_size: 100
      });
      const items = res.items || [];
      setData(items);

      // 计算汇总数据
      if (items.length > 0) {
        const avgQualifiedRate = items.reduce((sum, item) => sum + (item.quantity_qualified_rate || 0), 0) / items.length;
        const avgFirstPassYield = items.reduce((sum, item) => sum + (item.first_pass_yield || 0), 0) / items.length;
        const totalNcr = items.reduce((sum, item) => sum + (item.ncr_count || 0), 0);
        const totalQualityCost = items.reduce((sum, item) => sum + (item.total_quality_cost || 0), 0);
        setSummary({
          avgQualifiedRate,
          avgFirstPassYield,
          totalNcr,
          totalQualityCost
        });
      }
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
  }, [filters]);

  const chartData = {
    labels: data.map(item => dayjs(item.kpi_date).format("MM-DD")),
    datasets: [
      {
        label: "数量合格率",
        data: data.map(item => item.quantity_qualified_rate || 0),
        borderColor: "rgb(75, 192, 192)",
        backgroundColor: "rgba(75, 192, 192, 0.2)",
        tension: 0.1
      },
      {
        label: "一次合格率",
        data: data.map(item => item.first_pass_yield || 0),
        borderColor: "rgb(255, 99, 132)",
        backgroundColor: "rgba(255, 99, 132, 0.2)",
        tension: 0.1
      }
    ]
  };

  const costChartData = {
    labels: data.map(item => dayjs(item.kpi_date).format("MM-DD")),
    datasets: [
      {
        label: "质量成本",
        data: data.map(item => item.total_quality_cost || 0),
        backgroundColor: "rgba(54, 162, 235, 0.5)"
      }
    ]
  };

  const columns = [
    { title: "日期", dataIndex: "kpi_date", width: 120 },
    {
      title: "KPI类型",
      dataIndex: "kpi_type",
      width: 100,
      render: (v) => <Tag color={kpiTypeMap[v]?.color}>{kpiTypeMap[v]?.text || "未知"}</Tag>
    },
    {
      title: "数量合格率",
      dataIndex: "quantity_qualified_rate",
      width: 120,
      render: (v) => `${(v || 0).toFixed(2)}%`
    },
    {
      title: "一次合格率",
      dataIndex: "first_pass_yield",
      width: 120,
      render: (v) => `${(v || 0).toFixed(2)}%`
    },
    {
      title: "IQC合格率",
      dataIndex: "iqc_qualified_rate",
      width: 120,
      render: (v) => `${(v || 0).toFixed(2)}%`
    },
    {
      title: "IPQC合格率",
      dataIndex: "ipqc_qualified_rate",
      width: 120,
      render: (v) => `${(v || 0).toFixed(2)}%`
    },
    {
      title: "FQC合格率",
      dataIndex: "fqc_qualified_rate",
      width: 120,
      render: (v) => `${(v || 0).toFixed(2)}%`
    },
    {
      title: "返工率",
      dataIndex: "rework_rate",
      width: 100,
      render: (v) => `${(v || 0).toFixed(2)}%`
    },
    {
      title: "NCR数量",
      dataIndex: "ncr_count",
      width: 100
    },
    {
      title: "质量成本",
      dataIndex: "total_quality_cost",
      width: 120,
      render: (v) => `¥${(v || 0).toFixed(2)}`
    }
  ];

  return (
    <div>
      <Card
        title="质量KPI看板"
        extra={
          <Space>
            <DatePicker.RangePicker
              value={[dayjs(filters.start_date), dayjs(filters.end_date)]}
              onChange={(dates) => {
                if (dates) {
                  setFilters({
                    ...filters,
                    start_date: dates[0].format("YYYY-MM-DD"),
                    end_date: dates[1].format("YYYY-MM-DD")
                  });
                }
              }}
            />
            <Select
              placeholder="KPI类型"
              allowClear
              style={{ width: 150 }}
              onChange={(v) => setFilters({ ...filters, kpi_type: v })}
            >
              {Object.entries(kpiTypeMap).map(([k, v]) => (
                <Select.Option key={k} value={parseInt(k)}>{v.text}</Select.Option>
              ))}
            </Select>
          </Space>
        }
      >
        <Row gutter={16} style={{ marginBottom: 24 }}>
          <Col span={6}>
            <Card>
              <Statistic
                title="平均合格率"
                value={summary.avgQualifiedRate.toFixed(2)}
                suffix="%"
                valueStyle={{ color: summary.avgQualifiedRate >= 95 ? "#3f8600" : "#cf1322" }}
              />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic
                title="平均一次合格率"
                value={summary.avgFirstPassYield.toFixed(2)}
                suffix="%"
                valueStyle={{ color: summary.avgFirstPassYield >= 90 ? "#3f8600" : "#cf1322" }}
              />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic title="NCR总数" value={summary.totalNcr} suffix="件" />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic
                title="总质量成本"
                value={summary.totalQualityCost.toFixed(2)}
                prefix="¥"
              />
            </Card>
          </Col>
        </Row>

        <Row gutter={16} style={{ marginBottom: 24 }}>
          <Col span={12}>
            <Card title="合格率趋势">
              <Line data={chartData} options={{ responsive: true, maintainAspectRatio: false, height: 300 }} />
            </Card>
          </Col>
          <Col span={12}>
            <Card title="质量成本趋势">
              <Bar data={costChartData} options={{ responsive: true, maintainAspectRatio: false, height: 300 }} />
            </Card>
          </Col>
        </Row>

        <Card title="KPI明细数据">
          <Table
            columns={columns}
            dataSource={data}
            loading={loading}
            rowKey="id"
            scroll={{ x: 1200 }}
            pagination={{ pageSize: 20 }}
          />
        </Card>
      </Card>
    </div>
  );
}


