import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  clearScreen: false,
  server: {
    port: 8080,
    strictPort: true,
    proxy: {
      "/run-python-script": {
        target: "http://localhost:3000",
        changeOrigin: true
      }
    }
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: ["es2021", "chrome97", "safari13"],
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG
  },
  plugins: [vue()]
});
