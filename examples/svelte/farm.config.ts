import { defineConfig } from '@farmfe/core'
import { svelte } from '@farmfe/js-plugin-svelte'
import fs from 'fs'
export default defineConfig({
  plugins: [svelte(), base()],
  compilation: {
    persistentCache: false,
    progress: false,
  }
})

function base() {
  return {
    name: 'farm-load-vue-module-type',
    priority: -100,
    load: {
      filters: {
        resolvedPaths: ['.svelte'],
      },
      executor: async (param) => {
        const content = await fs.readFile(param.resolvedPath, 'utf-8')

        return {
          content,
          moduleType: 'js',
        }
      },
    },
  }
}
