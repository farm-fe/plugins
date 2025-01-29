import { defineConfig } from 'vite'
import preact from '@farmfe/js-plugin-preact'

// https://vite.dev/config/
export default defineConfig({
  plugins: [preact()],
})
