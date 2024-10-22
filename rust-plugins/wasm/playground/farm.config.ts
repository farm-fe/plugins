import { defineConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';
import farmPlugin from '@farmfe/plugin-wasm';

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    output: {
      filename: 'assets/[ext]/[name].[hash].[ext]',
      assetsFilename: 'static/[resourceName].[hash].[ext]'
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    react({ runtime: "automatic" }),
    farmPlugin()
  ],
});
