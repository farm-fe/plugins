import { defineConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';
import farmPlugin from 'farm-plugin-magic-string';
// console.log(__dirname);

export default defineConfig({
  compilation: {
    persistentCache: false,
    progress: false,
    runtime: {
      isolate: true
    }
  },
  plugins: [
    react(),
    farmPlugin()
  ],
});
