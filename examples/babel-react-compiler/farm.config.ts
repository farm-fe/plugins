import { defineConfig } from '@farmfe/core';
import react from '@farmfe/plugin-react';
import { reactCompiler } from '@farmfe/js-plugin-react-compiler';

export default defineConfig({
    compilation: {
        input: {},
        output: {},
        presetEnv: false,
        minify: false,
        mode: 'development',
        persistentCache: false,
    },
    plugins: [reactCompiler(), react()],
});
