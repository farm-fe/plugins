import { defineConfig } from "@farmfe/core";
import vue from "unplugin-vue/vite";
// import vue from "@vitejs/plugin-vue";
import vuetify, { transformAssetUrls } from "@farmfe/js-plugin-vuetify";
import component from "unplugin-vue-components/vite";
import path from "node:path";

export default defineConfig({
  plugins: [vuetify()],
  vitePlugins: [
    vue({
      template: { transformAssetUrls },
    }),
    component(),
  ],
  compilation: {
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "src"),
      },
    },
    persistentCache: false,
  },
});
