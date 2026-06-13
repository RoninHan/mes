/** SSO 前端地址（通过 VITE_SSO_URL 注入，默认本地开发端口 3001）*/
const SSO_URL = import.meta.env.VITE_SSO_URL ?? "http://localhost:3001";
/** MES 在 SSO 中注册的 app_key */
const APP_KEY = import.meta.env.VITE_MES_APP_KEY ?? "mes-app";
/** MES SSO 回调地址 */
const REDIRECT_URI = `${window.location.origin}/auth/callback`;

/**
 * 跳转至 SSO 登录页（同时支持账号密码 + 飞书扫码登录）。
 * SSO 登录成功后重定向到 /auth/callback?sso_token=<token>
 */
export function redirectToSsoLogin() {
  const url = new URL(`${SSO_URL}/login`);
  url.searchParams.set("app_key", APP_KEY);
  url.searchParams.set("redirect_uri", REDIRECT_URI);
  window.location.href = url.toString();
}

/**
 * 从 URL 参数中提取 sso_token 并存入 localStorage。
 * @returns {string|null} token 字符串，或 null（参数不存在时）
 */
export function extractSsoToken() {
  const params = new URLSearchParams(window.location.search);
  const token = params.get("sso_token");
  if (!token) return null;
  localStorage.setItem("token", token);
  // 清理 URL 中的敏感参数
  window.history.replaceState({}, "", window.location.pathname);
  return token;
}

/** 退出登录：清除本地 token 并跳回登录页 */
export function logout() {
  localStorage.removeItem("token");
  localStorage.removeItem("user");
  window.location.href = "/login";
}


