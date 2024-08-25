import { defineConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';
import farmPlugin from 'farm-plugin-react-router';

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
    farmPlugin()
  ],
});
