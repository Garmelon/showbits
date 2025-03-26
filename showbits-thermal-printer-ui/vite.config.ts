import vue from "@vitejs/plugin-vue";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import vueDevTools from "vite-plugin-vue-devtools";

const root = dirname(fileURLToPath(import.meta.url));
function path(to: string): string {
  return resolve(root, to);
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue(), vueDevTools()],
  base: "",
  build: {
    rollupOptions: {
      input: [path("index.html"), path("photo.html")],
    },
  },
  resolve: {
    alias: { "@": path("src") },
  },
  server: {
    proxy: { "/api": "http://localhost:8080" },
  },
});
