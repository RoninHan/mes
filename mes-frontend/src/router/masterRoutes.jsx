import React from "react";
import MaterialCategoryList from "../pages/MasterData/MaterialCategoryList";
import MaterialsList from "../pages/MasterData/MaterialsList";
import SuppliersList from "../pages/MasterData/SuppliersList";
import CustomersList from "../pages/MasterData/CustomersList";

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
  }
];


