import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { ConfigProvider } from "antd";
import { Provider } from "react-redux";
import zhCN from "antd/locale/zh_CN";
import AppRouter from "./router/AppRouter";
import Login from "./pages/Auth/Login";
import "./i18n";
import { store } from "./store";

function RootRouter() {
  const token = localStorage.getItem("token");
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/login" element={<Login />} />
        <Route
          path="/*"
          element={token ? <AppRouter /> : <Navigate to="/login" replace />}
        />
      </Routes>
    </BrowserRouter>
  );
}

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <ConfigProvider locale={zhCN} theme={{ token: { colorPrimary: "#1677ff" } }}>
      <Provider store={store}>
        <RootRouter />
      </Provider>
    </ConfigProvider>
  </React.StrictMode>
);


