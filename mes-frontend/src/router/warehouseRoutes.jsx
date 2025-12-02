import React from "react";
import InventoryList from "../pages/Warehouse/InventoryList";
import InboundOrderList from "../pages/Warehouse/InboundOrderList";

export default [
  {
    path: "/warehouse/inventory",
    element: <InventoryList />
  },
  {
    path: "/warehouse/inbound-orders",
    element: <InboundOrderList />
  }
];


