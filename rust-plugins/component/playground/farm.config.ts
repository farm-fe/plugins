import { defineConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    react({ runtime: "automatic" }),
    ['@farmfe/plugin-component', {
      library_name: 'antd',
      lib_dir: 'es',
      camel2_dash: false,
      style_lib_dir: 'lib',
      style_library_name: 'theme-default',
      style_library_path: '[module]/style/index.css'
    }]
  ],
});
