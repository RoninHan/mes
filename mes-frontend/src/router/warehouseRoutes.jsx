import React from "react";
import InventoryList from "../pages/Warehouse/InventoryList";
import InboundOrderList from "../pages/Warehouse/InboundOrderList";
import OutboundOrderList from "../pages/Warehouse/OutboundOrderList";
import TransferOrderList from "../pages/Warehouse/TransferOrderList";
import StockCountOrderList from "../pages/Warehouse/StockCountOrderList";

export default [
  {
    path: "/warehouse/inventory",
    element: <InventoryList />
  },
  {
    path: "/warehouse/inbound-orders",
    element: <InboundOrderList />
  },
  {
    path: "/warehouse/outbound-orders",
    element: <OutboundOrderList />
  },
  {
    path: "/warehouse/transfer-orders",
    element: <TransferOrderList />
  },
  {
    path: "/warehouse/stock-count-orders",
    element: <StockCountOrderList />
  }
];


