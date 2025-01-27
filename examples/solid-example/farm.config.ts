import { defineConfig } from '@farmfe/core'
import solid from '@farmfe/js-plugin-solid'

export default defineConfig({
  plugins: [solid({
  })],
  compilation: {
    persistentCache: false,
    progress: false,
  }
})
