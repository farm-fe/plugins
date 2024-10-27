import { defineConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';
import farmPlugin from '@farmfe/plugin-worker';
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
    farmPlugin({
      compilerConfig:{
        output:{
          assetsFilename: '[resourceName].worker.[hash].[ext]',
        }
      }
    })
  ],
});
