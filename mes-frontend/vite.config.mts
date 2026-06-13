import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

export default defineConfig({
  plugins: [
    // include .js/.jsx/.ts/.tsx so files with JSX kept as .js work without renaming
    react({ include: "**/*.{js,jsx,ts,tsx}" }),
  ],
  server: {
    port: 5173,
    proxy: {
      "/api": {
        target: "http://localhost:8080",
        changeOrigin: true
      }
    }
  }
});


