import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { extname } from 'node:path';
import * as babel from '@babel/core';
import ts from '@babel/preset-typescript';
import { createFilter } from '@rollup/pluginutils';
import solid from 'babel-preset-solid';
import { mergeAndConcat } from 'merge-anything';
import solidRefresh from 'solid-refresh/babel';

import type { JsPlugin } from '@farmfe/core';
import type { Options } from './types.ts';

const require = createRequire(import.meta.url);

const runtimePublicPath = '/@solid-refresh';
const runtimeFilePath = require.resolve('solid-refresh/dist/solid-refresh.mjs');
const runtimeCode = readFileSync(runtimeFilePath, 'utf-8');

function tryToReadFileSync(path: string) {
  try {
    return readFileSync(path, 'utf-8');
  } catch (error) {
    console.error(`[Farm Plugin Solid]: ${error.type}: ${error.message}`);
  }
}

export default function farmPluginSolid(
  options: Partial<Options> = {}
): JsPlugin {
  const filter = createFilter(options.include, options.exclude);

  let needHmr = false;
  let replaceDev = false;
  let projectRoot = process.cwd();

  const extensionsToWatch = [...(options.extensions ?? []), '.tsx', '.jsx'];
  const allExtensions = extensionsToWatch.map((extension) =>
    // An extension can be a string or a tuple [extension, options]
    typeof extension === 'string' ? extension : extension[0]
  );

  return {
    name: 'farm-plugin-solid',
    async config(userConfig, { command }) {
      // We inject the dev mode only if the user explicitely wants it or if we are in dev (serve) mode
      replaceDev = options.dev === true || (options.dev !== false && command === 'dev');
      projectRoot = userConfig.root;

      if (!userConfig.compilation.resolve) userConfig.compilation.resolve = {};

      userConfig.compilation.resolve.alias = normalizeAliases(userConfig.compilation.resolve?.alias ?? []);

      // fix for bundling dev in production
      const nestedDeps = replaceDev
        ? ['solid-js', 'solid-js/web', 'solid-js/store', 'solid-js/html', 'solid-js/h']
        : [];



      return {
        compilation: {
          resolve: {
            conditions:
              [
                'solid',
                ...(replaceDev ? ['development'] : []),
              ],
            dedupe: nestedDeps,
            alias: [{ find: '/^solid-refresh$/', replacement: runtimePublicPath }],
          },
        }
      };
    },

    configResolved(config) {
      needHmr = config.command === 'dev' && config.compilation.mode !== 'production' && options.hot !== false;
    },
    load: {
      filters: {
        resolvedPaths: [...allExtensions, runtimePublicPath]
      },
      async executor(param) {
        if (param.resolvedPath === runtimePublicPath) {
          return {
            content: runtimeCode,
            moduleType: 'solid-refresh'
          };
        }

        const source = tryToReadFileSync(param.resolvedPath);

        return {
          content: source,
          moduleType: 'solid'
        };
      }
    },
    transform: {
      filters: {
        moduleTypes: ['solid', 'solid-refresh']
      },
      async executor(param) {
        const isSsr = options.ssr;
        const currentFileExtension = extname(param.resolvedPath);

        if (!filter(param.resolvedPath) || !(/\.[mc]?[tj]sx$/i.test(param.resolvedPath) || allExtensions.includes(currentFileExtension))) {
          return;
        }

        let solidOptions: { generate: 'ssr' | 'dom'; hydratable: boolean };

        if (options.ssr) {
          if (isSsr) {
            solidOptions = { generate: 'ssr', hydratable: true };
          } else {
            solidOptions = { generate: 'dom', hydratable: true };
          }
        } else {
          solidOptions = { generate: 'dom', hydratable: false };
        }

        param.resolvedPath = param.resolvedPath.replace(/\?.+$/, '');

        // We need to know if the current file extension has a typescript options tied to it
        const shouldBeProcessedWithTypescript = /\.[mc]?tsx$/i.test(param.resolvedPath) || extensionsToWatch.some((extension) => {
          if (typeof extension === 'string') {
            return extension.includes('tsx');
          }

          const [extensionName, extensionOptions] = extension;
          if (extensionName !== currentFileExtension) return false;

          return extensionOptions.typescript;
        });
        const plugins: NonNullable<NonNullable<babel.TransformOptions['parserOpts']>['plugins']> = ['jsx']

        if (shouldBeProcessedWithTypescript) {
          plugins.push('typescript');
        }

        const opts: babel.TransformOptions = {
          root: projectRoot,
          filename: param.resolvedPath,
          sourceFileName: param.resolvedPath,
          presets: [[solid, { ...solidOptions, ...(options.solid || {}) }]],
          plugins: needHmr && !isSsr ? [[solidRefresh, { bundler: 'vite' }]] : [],
          ast: false,
          sourceMaps: true,
          configFile: false,
          babelrc: false,
          parserOpts: {
            plugins,
          },
        };

        if (shouldBeProcessedWithTypescript) {
          opts.presets.push([ts, options.typescript ?? {}]);
        }

        // Default value for babel user options
        let babelUserOptions: babel.TransformOptions = {};

        if (options.babel) {
          if (typeof options.babel === 'function') {
            const babelOptions = options.babel(param.content, param.resolvedPath, isSsr);
            babelUserOptions = babelOptions instanceof Promise ? await babelOptions : babelOptions;
          } else {
            babelUserOptions = options.babel;
          }
        }

        const babelOptions = mergeAndConcat(babelUserOptions, opts) as babel.TransformOptions;

        const { code, map } = await babel.transformAsync(
          param.content,
          babelOptions
        );

        return {
          content: code,
          sourceMap: JSON.stringify(map),
          moduleType: 'js'
        };
      }
    }
  };
}

/**
 * This basically normalize all aliases of the config into
 * the array format of the alias.
 *
 * eg: alias: { '@': 'src/' } => [{ find: '@', replacement: 'src/' }]
 */
function normalizeAliases(alias) {
  return Array.isArray(alias)
    ? alias
    : Object.entries(alias).map(([find, replacement]) => ({ find, replacement }));
}

