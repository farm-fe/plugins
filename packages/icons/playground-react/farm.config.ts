import { defineConfig } from "@farmfe/core";
import farmJsPluginPostcss from '@farmfe/js-plugin-postcss';
import visualizer from '@farmfe/js-plugin-visualizer'
export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    farmJsPluginPostcss(),
    visualizer(),
    ["@farmfe/plugin-react", { runtime: "automatic" }],
    ["@farmfe/plugin-icons", {
      autoInstall: true,
      compiler: "jsx",
    }],
  ],
});
