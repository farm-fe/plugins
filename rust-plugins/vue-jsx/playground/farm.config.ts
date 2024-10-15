import { defineConfig } from '@farmfe/core';
import vue from '@vitejs/plugin-vue';
import vueJsx from '@farmfe/plugin-vue-jsx';

export default defineConfig({
  vitePlugins: [vue()],
  plugins: [vueJsx()]
});
