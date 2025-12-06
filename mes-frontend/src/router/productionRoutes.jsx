import React from "react";
import ProductionPlanList from "../pages/Production/ProductionPlanList";
import ProductionOrderList from "../pages/Production/ProductionOrderList";
import WorkOrderList from "../pages/Production/WorkOrderList";
import ProductionReportList from "../pages/Production/ProductionReportList";
import ProductionSchedule from "../pages/Production/ProductionSchedule";
import MaterialRequirementList from "../pages/Production/MaterialRequirementList";
import PickingOrderList from "../pages/Production/PickingOrderList";
import ReturnOrderList from "../pages/Production/ReturnOrderList";
import ProductionReceiptList from "../pages/Production/ProductionReceiptList";

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
  },
  {
    path: "/production/material-requirements",
    element: <MaterialRequirementList />
  },
  {
    path: "/production/picking-orders",
    element: <PickingOrderList />
  },
  {
    path: "/production/return-orders",
    element: <ReturnOrderList />
  },
  {
    path: "/production/receipts",
    element: <ProductionReceiptList />
  }
];
