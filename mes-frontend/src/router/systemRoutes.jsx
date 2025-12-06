import React from "react";
import UserList from "../pages/System/UserList";
import LoginLogList from "../pages/System/LoginLogList";
import OperationLogList from "../pages/System/OperationLogList";

export default [
  {
    path: "/system/user",
    element: <UserList />
  },
  {
    path: "/system/login-logs",
    element: <LoginLogList />
  },
  {
    path: "/system/operation-logs",
    element: <OperationLogList />
  }
];


