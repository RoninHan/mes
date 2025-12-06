import React from "react";
import EquipmentList from "../pages/Equipment/EquipmentList";
import EquipmentDetail from "../pages/Equipment/EquipmentDetail";
import EquipmentEdit from "../pages/Equipment/EquipmentEdit";
import EquipmentStatusLog from "../pages/Equipment/EquipmentStatusLog";
import EquipmentControl from "../pages/Equipment/EquipmentControl";
import MaintenancePlanList from "../pages/Equipment/MaintenancePlanList";
import MaintenanceTaskList from "../pages/Equipment/MaintenanceTaskList";
import FaultReportList from "../pages/Equipment/FaultReportList";
import RepairOrderList from "../pages/Equipment/RepairOrderList";
import InspectionList from "../pages/Equipment/InspectionList";
import EquipmentKpiDashboard from "../pages/Equipment/EquipmentKpiDashboard";

export default [
  {
    path: "/equipment/list",
    element: <EquipmentList />
  },
  {
    path: "/equipment/:id",
    element: <EquipmentDetail />
  },
  {
    path: "/equipment/:id/edit",
    element: <EquipmentEdit />
  },
  {
    path: "/equipment/status-log",
    element: <EquipmentStatusLog />
  },
  {
    path: "/equipment/control",
    element: <EquipmentControl />
  },
  {
    path: "/equipment/maintenance-plans",
    element: <MaintenancePlanList />
  },
  {
    path: "/equipment/maintenance-tasks",
    element: <MaintenanceTaskList />
  },
  {
    path: "/equipment/fault-reports",
    element: <FaultReportList />
  },
  {
    path: "/equipment/repair-orders",
    element: <RepairOrderList />
  },
  {
    path: "/equipment/inspections",
    element: <InspectionList />
  },
  {
    path: "/equipment/kpi",
    element: <EquipmentKpiDashboard />
  }
];


