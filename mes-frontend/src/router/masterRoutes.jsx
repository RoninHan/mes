import React from "react";
import MaterialCategoryList from "../pages/MasterData/MaterialCategoryList";
import MaterialsList from "../pages/MasterData/MaterialsList";
import SuppliersList from "../pages/MasterData/SuppliersList";
import CustomersList from "../pages/MasterData/CustomersList";
import WorkshopList from "../pages/MasterData/WorkshopList";
import WarehouseList from "../pages/MasterData/WarehouseList";
import LocationList from "../pages/MasterData/LocationList";
import BomList from "../pages/MasterData/BomList";
import ProcessRouteList from "../pages/MasterData/ProcessRouteList";

export default [
  {
    path: "/master/material-categories",
    element: <MaterialCategoryList />
  },
  {
    path: "/master/materials",
    element: <MaterialsList />
  },
  {
    path: "/master/suppliers",
    element: <SuppliersList />
  },
  {
    path: "/master/customers",
    element: <CustomersList />
  },
  {
    path: "/master/workshops",
    element: <WorkshopList />
  },
  {
    path: "/master/warehouses",
    element: <WarehouseList />
  },
  {
    path: "/master/locations",
    element: <LocationList />
  },
  {
    path: "/master/boms",
    element: <BomList />
  },
  {
    path: "/master/routes",
    element: <ProcessRouteList />
  }
];


