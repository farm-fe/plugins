import { defineConfig } from "@farmfe/core";

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    ["farm-plugin-react-components",{}],
    ["@farmfe/plugin-react", { runtime: "automatic" }],
  ],
});
