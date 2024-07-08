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
    ["@farmfe/plugin-react", { runtime: "automatic" }],
    ["@farmfe/plugin-icons", {
      autoInstall: true,
      compiler: "jsx",
      defaultStyle: {
        width: "2em",
        height: "2em",
      },
      defaultClass:"icon"
    }],
  ],
});
