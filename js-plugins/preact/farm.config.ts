import { defineConfig } from '@farmfe/core';

const format = (process.env.FARM_FORMAT as 'esm' | 'cjs') || 'esm';
const ext = format === 'esm' ? 'mjs' : 'cjs';

export default defineConfig({
	watch: true,
	compilation: {
		external: ["@farmfe/core", "@babel/core", "vite"],
		input: {
			index: './src/index.ts'
		},
		output: {
			path: `dist/${format}`,
			entryFilename: `[entryName].${ext}`,
			targetEnv: 'node',
			format
		},
		partialBundling: {
			enforceResources: [
				{
					name: 'index',
					test: ['.+']
				}
			]
		},
		minify: false,
		sourcemap: false,
		presetEnv: false,
		persistentCache: {
			envs: {
				FARM_FORMAT: format
			}
		}
	},
});
