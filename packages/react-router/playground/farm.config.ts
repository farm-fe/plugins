import { defineConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';
import pluginReactRouter from '@farmfe/plugin-react-router';

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
    pluginReactRouter()
  ],
});
