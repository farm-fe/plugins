import { defineConfig, UserConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';
import farmPlugin from '@farmfe/plugin-worker';
export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    minify: false,
    persistentCache: true,
    progress: false,
  },
  plugins: [
    react({ runtime: "automatic" }),
    farmPlugin({
      isBuild: false,
      compilerConfig:{
        presetEnv: true,
        output:{
          assetsFilename: 'asserts/[resourceName].[hash].[ext]',
        },
        minify: false,
        treeShaking: false,
      } as UserConfig['compilation']
    })
  ],
});
