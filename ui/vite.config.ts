import { defineConfig } from "vite";

// frontendDist in app-tauri/tauri.conf.json points at ../ui/dist
export default defineConfig({
  root: ".",
  build: {
    outDir: "dist",
    emptyOutDir: true,
    target: "es2022",
    sourcemap: false,
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
});
