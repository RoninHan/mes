import React from "react";
import ProductionPlanList from "../pages/Production/ProductionPlanList";
import ProductionOrderList from "../pages/Production/ProductionOrderList";
import WorkOrderList from "../pages/Production/WorkOrderList";
import ProductionReportList from "../pages/Production/ProductionReportList";
import ProductionSchedule from "../pages/Production/ProductionSchedule";

export default [
  {
    path: "/production/plans",
    element: <ProductionPlanList />
  },
  {
    path: "/production/orders",
    element: <ProductionOrderList />
  },
  {
    path: "/production/work-orders",
    element: <WorkOrderList />
  },
  {
    path: "/production/reports",
    element: <ProductionReportList />
  },
  {
    path: "/production/schedule",
    element: <ProductionSchedule />
  }
];
