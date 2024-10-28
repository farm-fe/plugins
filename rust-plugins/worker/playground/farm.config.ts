import { defineConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';
import farmPlugin from '@farmfe/plugin-worker';
export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    minify: false,
    persistentCache: false,
    progress: false,
  },
  plugins: [
    react({ runtime: "automatic" }),
    farmPlugin({
      isBuild: true,
      compilerConfig: {
        output: {
          assetsFilename: 'worker/[resourceName].worker.[hash].[ext]',
        },
      }
    })
  ],
});
