import { defineConfig } from '@farmfe/core';

export default defineConfig({
  plugins: ['@farmfe/plugin-react', ["farm-plugin-virtual", {
    virtual: "export const text = 'this is a virtual module'"
  }]]
});
