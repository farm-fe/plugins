import { defineConfig } from "@farmfe/core";
import path from 'node:path';
import vue from '@vitejs/plugin-vue2';
export default defineConfig({
  compilation: {
    // persistentCache: false,
    progress: false,
    resolve: {
      alias: {
        '@': path.resolve(process.cwd(), 'src'),
      }
    }
  },
  vitePlugins: [vue()],
  plugins: [
    ['@farmfe/plugin-component', {
      library_name: 'element-ui',
      lib_dir: 'lib',
      camel2_dash: false,
      style_lib_dir: 'lib',
      style_library_name: 'theme-chalk',
      style_library_path: '.css'
    }]
  ],
});
