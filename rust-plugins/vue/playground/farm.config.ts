import { defineConfig } from '@farmfe/core';
import _vue from '@vitejs/plugin-vue';
import vue from '@farmfe/plugin-vue';
import visualizer from "@farmfe/js-plugin-visualizer"
export default defineConfig({
  vitePlugins: [
    // vue(),
  ],
  plugins: [
    vue(),
    visualizer(),
  ],
});
