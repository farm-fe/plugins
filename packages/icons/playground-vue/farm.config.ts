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
      // 支持本地 svg 
      autoInstall: true,
      compiler: 'vue',
      defaultStyle:{
        width:"2em",
        height:"2em",
      }
    }],
  ]
});
