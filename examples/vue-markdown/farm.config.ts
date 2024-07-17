import { defineConfig } from '@farmfe/core';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  vitePlugins: [vue({
    include: [/\.vue$/, /\.md$/],
  })],
  plugins: ["@farmfe/plugin-vue-markdown"],
  compilation: {
    persistentCache: false
  }
});
