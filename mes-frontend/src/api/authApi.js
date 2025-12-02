import request from "./axiosInstance";

export const login = (data) => request.post("/auth/login", data);


