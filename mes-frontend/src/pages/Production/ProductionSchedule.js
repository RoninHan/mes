import React, { useEffect, useState } from "react";
import { Card, DatePicker, Space, Select, Button, message } from "antd";
import { fetchScheduleTimeline, updateSchedule, runAutoSchedule } from "../../api/scheduleApi";
import dayjs from "dayjs";

const { RangePicker } = DatePicker;

// 一个简单的“甘特条”渲染：把一天拆成 24 格，按时间位置和跨度渲染条形块
function GanttRow({ item, range, onChangeTime }) {
  const [moving, setMoving] = useState(false);

  const totalMs = range[1].valueOf() - range[0].valueOf();
  const startMs = dayjs(item.start_time).valueOf() - range[0].valueOf();
  const endMs = dayjs(item.end_time).valueOf() - range[0].valueOf();

  const leftPercent = Math.max(0, Math.min(100, (startMs / totalMs) * 100));
  const widthPercent = Math.max(2, ((endMs - startMs) / totalMs) * 100);

  const handleDrag = (e) => {
    if (!moving) return;
    const rowRect = e.currentTarget.parentElement.getBoundingClientRect();
    const delta = e.clientX - rowRect.left;
    const newStart = range[0].valueOf() + (delta / rowRect.width) * totalMs;
    const duration = endMs - startMs;
    const newEnd = newStart + duration;
    onChangeTime(dayjs(newStart).toISOString(), dayjs(newEnd).toISOString());
  };

  return (
    <div
      style={{ position: "relative", height: 28, borderBottom: "1px solid #f0f0f0" }}
      onMouseMove={handleDrag}
      onMouseUp={() => setMoving(false)}
      onMouseLeave={() => setMoving(false)}
    >
      <div
        style={{
          position: "absolute",
          left: `${leftPercent}%`,
          width: `${widthPercent}%`,
          top: 4,
          bottom: 4,
          background: "#1677ff",
          borderRadius: 4,
          color: "#fff",
          fontSize: 12,
          paddingLeft: 4,
          cursor: "move",
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis"
        }}
        onMouseDown={() => setMoving(true)}
      >
        {item.work_order_no}
      </div>
    </div>
  );
}

export default function ProductionSchedule() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [range, setRange] = useState([dayjs().startOf("day"), dayjs().add(1, "day")]);
  const [workshopId, setWorkshopId] = useState();
  const [equipmentId, setEquipmentId] = useState();

  const load = async () => {
    setLoading(true);
    try {
      const res = await fetchScheduleTimeline({
        workshop_id: workshopId,
        equipment_id: equipmentId,
        from: range[0].toISOString(),
        to: range[1].toISOString(),
        page: 0,
        page_size: 200
      });
      setData(res.items || []);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleTimeChange = async (id, start, end) => {
    try {
      await updateSchedule(id, { start_time: start, end_time: end });
      message.success("排程已更新");
      load();
    } catch {
      message.error("更新排程失败");
    }
  };

  const handleRunAuto = async () => {
    try {
      await runAutoSchedule({
        workshop_id: workshopId,
        equipment_id: equipmentId,
        from: range[0].toISOString()
      });
      message.success("已执行简单自动排程");
      load();
    } catch {
      message.error("自动排程失败");
    }
  };

  return (
    <Card
      title="生产排程"
      loading={loading}
      extra={
        <Space>
          <RangePicker
            value={range}
            onChange={(vals) => {
              if (vals) {
                setRange(vals);
              }
            }}
          />
          <Select
            placeholder="车间ID"
            allowClear
            style={{ width: 120 }}
            value={workshopId}
            onChange={setWorkshopId}
            options={[
              { value: 1, label: "车间1" },
              { value: 2, label: "车间2" }
            ]}
          />
          <InputNumber
            placeholder="设备ID"
            style={{ width: 120 }}
            value={equipmentId}
            onChange={setEquipmentId}
          />
          <Button type="primary" onClick={load}>
            查询
          </Button>
          <Button onClick={handleRunAuto}>自动排程</Button>
        </Space>
      }
    >
      <div style={{ display: "flex", border: "1px solid #f0f0f0" }}>
        <div style={{ width: 220, borderRight: "1px solid #f0f0f0" }}>
          <div style={{ padding: "4px 8px", fontWeight: 500 }}>工单</div>
          {data.map((item) => (
            <div
              key={item.id}
              style={{
                height: 28,
                display: "flex",
                alignItems: "center",
                borderBottom: "1px solid #f0f0f0",
                padding: "0 8px",
                fontSize: 12
              }}
            >
              {item.work_order_no}
            </div>
          ))}
        </div>
        <div style={{ flex: 1 }}>
          <div
            style={{
              display: "flex",
              fontSize: 10,
              borderBottom: "1px solid #f0f0f0",
              background: "#fafafa"
            }}
          >
            {Array.from({ length: 24 }).map((_, i) => (
              <div key={i} style={{ flex: 1, textAlign: "center", padding: "2px 0" }}>
                {i}:00
              </div>
            ))}
          </div>
          {data.map((item) => (
            <GanttRow
              key={item.id}
              item={item}
              range={range}
              onChangeTime={(start, end) => handleTimeChange(item.id, start, end)}
            />
          ))}
        </div>
      </div>
    </Card>
  );
}


