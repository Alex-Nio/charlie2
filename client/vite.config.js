import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  clearScreen: false,
  server: {
    port: 8080,
    strictPort: true
  },
  envPrefix: ["VITE_", "TAURI_"],
  // build: {
  //   target: ["es2021", "chrome97", "safari13"],
  //   minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
  //   sourcemap: !!process.env.TAURI_DEBUG
  // },
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG
  },
  plugins: [vue()]
});
