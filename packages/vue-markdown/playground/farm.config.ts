import { defineConfig } from "@farmfe/core";
import Markdown from "unplugin-vue-markdown/vite"

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
  ],
  vitePlugins: [Markdown({})]
});
