import { defineConfig } from '@farmfe/core';
import farmfePluginVVirtual from '@farmfe/plugin-virtual'

export default defineConfig({
  plugins: ['@farmfe/plugin-react', farmfePluginVVirtual({
    "virtual": "export const a = 1",
  })]
});
