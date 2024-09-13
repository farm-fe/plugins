import { defineConfig } from "@farmfe/core";
import farmPluginMdx from "@farmfe/plugin-mdx";

export default defineConfig({
  plugins: ["@farmfe/plugin-react", farmPluginMdx()],
});
