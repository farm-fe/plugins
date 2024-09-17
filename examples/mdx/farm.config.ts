import { defineConfig } from "@farmfe/core";
import farmPluginMdx from "@farmfe/plugin-mdx";
import { jsx } from "react/jsx-runtime";

export default defineConfig({
  plugins: ["@farmfe/plugin-react", farmPluginMdx({
    jsx: true
  })],
  compilation: {
    persistentCache: false,
  },
});
