import { defineConfig } from "@farmfe/core";
import { resolve } from "path"
export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    ["@farmfe/plugin-react", { runtime: "automatic" }],
    [
      "@farmfe/plugin-url", { limit: 10 * 1024, public_path: "output/", emit_files: true, dest_dir: resolve(__dirname, "./dist/assets") }
    ]
  ],
});
