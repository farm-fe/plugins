import { defineConfig } from "@farmfe/core";
import less from "@farmfe/js-plugin-less"

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    less(),
    ["@farmfe/plugin-react", { runtime: "automatic" }],
    ["@farmfe/plugin-react-components", {
      dts: true,
      local: true,
      filename: "src/types/components.d.ts",
      resolvers: [
        {
          module: "antd",
          prefix: "Ant"
        },
        {
          module: "@arco-design/web-react",
          prefix: "Arco",
          import_style: true // style/index.js
        }
      ]
    }]
  ],
});
