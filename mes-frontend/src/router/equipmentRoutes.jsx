import React from "react";
import EquipmentList from "../pages/Equipment/EquipmentList";
import EquipmentDetail from "../pages/Equipment/EquipmentDetail";
import EquipmentEdit from "../pages/Equipment/EquipmentEdit";
import EquipmentStatusLog from "../pages/Equipment/EquipmentStatusLog";
import EquipmentControl from "../pages/Equipment/EquipmentControl";

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
  }
];


