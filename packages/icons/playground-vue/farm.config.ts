import { defineConfig } from '@farmfe/core';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
  },
  vitePlugins: [vue()],
  plugins: [
    ["@farmfe/plugin-icons", {
      autoInstall: true,
      compiler: 'vue'
    }],
  ]
});
