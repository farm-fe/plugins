import { defineConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';
import autoImport from '@farmfe/plugin-auto-import';
import visualizer from '@farmfe/js-plugin-visualizer';

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
    autoImport({
      presets: [
        "react",
        "react-router",
        {
          '@vueuse/core': [
            'useMouse',
            ['useFetch', 'useMyFetch']
          ],
        },
        {
          from: 'vue-router',
          imports: ['RouteLocationRaw'],
        },
      ],
      dirs: ["src/apis"],
      ignore:[]
    }),
    visualizer()
  ],
});
