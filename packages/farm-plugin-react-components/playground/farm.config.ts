import { defineConfig } from "@farmfe/core";
import farmJsPluginLess from '@farmfe/js-plugin-less';
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
    ["farm-plugin-react-components", {
      resolvers: [
        {
          module: "antd",
          prefix: "Ant",
        },
        {
          module: "@arco-design/web-react",
          prefix: "A",
          import_style: ''
        }
      ]
    }],
    farmJsPluginLess(),
  ],
});
