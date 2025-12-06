import React from "react";
import InspectionTaskList from "../pages/Quality/InspectionTaskList";
import InspectionReportList from "../pages/Quality/InspectionReportList";
import NcrList from "../pages/Quality/NcrList";
import ComplaintList from "../pages/Quality/ComplaintList";
import ReworkOrderList from "../pages/Quality/ReworkOrderList";
import MeasuringEquipmentList from "../pages/Quality/MeasuringEquipmentList";
import SupplierEvaluationList from "../pages/Quality/SupplierEvaluationList";
import TraceabilityRecordList from "../pages/Quality/TraceabilityRecordList";
import QualityCostList from "../pages/Quality/QualityCostList";
import QualityKpiDashboard from "../pages/Quality/QualityKpiDashboard";

export default [
  {
    path: "/quality/inspection-tasks",
    element: <InspectionTaskList />
  },
  {
    path: "/quality/inspection-reports",
    element: <InspectionReportList />
  },
  {
    path: "/quality/ncr",
    element: <NcrList />
  },
  {
    path: "/quality/complaints",
    element: <ComplaintList />
  },
  {
    path: "/quality/rework-orders",
    element: <ReworkOrderList />
  },
  {
    path: "/quality/measuring-equipment",
    element: <MeasuringEquipmentList />
  },
  {
    path: "/quality/supplier-evaluations",
    element: <SupplierEvaluationList />
  },
  {
    path: "/quality/traceability",
    element: <TraceabilityRecordList />
  },
  {
    path: "/quality/costs",
    element: <QualityCostList />
  },
  {
    path: "/quality/kpi",
    element: <QualityKpiDashboard />
  }
];

