import { defineConfig } from '@farmfe/core';
import virtual from "@farmfe/plugin-virtual"

export default defineConfig({
  plugins: ['@farmfe/plugin-react', [virtual, {
    'virtualModule': 'export const a = "virtual-module"',
  }]]
});
