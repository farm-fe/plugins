import { defineConfig } from "@farmfe/core";
import farmPluginMcp from "@farmfe/plugin-mcp"

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
    farmPluginMcp({})
  ],
});
