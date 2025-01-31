import fs from "node:fs";
import process from "node:process";
// import { svelteInspector } from '@sveltejs/vite-plugin-svelte-inspector';
import { handleHotUpdate } from "./handle-hot-update.js";
import { log, logCompilerWarnings } from "./utils/log.js";
import { createCompileSvelte } from "./utils/compile.js";
import { buildIdParser, buildModuleIdParser } from "./utils/id.js";
import {
  validateInlineOptions,
  resolveOptions,
  patchResolvedViteConfig,
  preResolveOptions,
  ensureConfigEnvironmentMainFields,
  ensureConfigEnvironmentConditions,
  buildExtraFarmConfig,
} from "./utils/options.js";
import { ensureWatchedFile, setupWatchers } from "./utils/watch.js";
import { toRollupError } from "./utils/error.js";
import { saveSvelteMetadata } from "./utils/optimizer.js";
import { VitePluginSvelteCache } from "./utils/vite-plugin-svelte-cache.js";
import { loadRaw } from "./utils/load-raw.js";
import * as svelteCompiler from "svelte/compiler";

/**
 * @param {Partial<import('./public.d.ts').Options>} [inlineOptions]
 * @returns {import('vite').Plugin[]}
 */
export function svelte(inlineOptions) {
  if (process.env.DEBUG != null) {
    log.setLevel("debug");
  }
  validateInlineOptions(inlineOptions);
  const cache = new VitePluginSvelteCache();
  // updated in configResolved hook
  /** @type {import('./types/id.d.ts').IdParser} */
  let requestParser;
  /** @type {import('./types/id.d.ts').ModuleIdParser} */
  let moduleRequestParser;
  /** @type {import('./types/options.d.ts').ResolvedOptions} */
  let options;
  /** @type {import('vite').ResolvedConfig} */
  let viteConfig;
  /** @type {import('./types/compile.d.ts').CompileSvelte} */
  let compileSvelte;
  const plugins = [
    {
      name: "svelte",
      // make sure our resolver runs before vite internal resolver to resolve svelte field correctly
      priority: 105,
      async config(config, configEnv) {
        options = await preResolveOptions(inlineOptions, config, configEnv);

        const extraFarmConfig = await buildExtraFarmConfig();

        return extraFarmConfig;
      },

      async configResolved(config) {
        options = resolveOptions(options, config, cache);

        requestParser = buildIdParser(options);

        compileSvelte = createCompileSvelte();
        viteConfig = config;
      },
      configureServer(server) {
        options.server = server;
        setupWatchers(options, cache, requestParser);
      },

      load: {
        filters: {
          resolvedPaths: [".*"],
        },
        async executor(param) {
          const isSsr = options.ssr ?? false;
          const svelteRequest = requestParser(
            param.resolvedPath,
            !!options.ssr
          );
          if (svelteRequest) {
            const { filename, query, raw } = svelteRequest;
            if (raw) {
              console.log(filename);

              const code = await loadRaw(svelteRequest, compileSvelte, options);
              // prevent vite from injecting sourcemaps in the results.
              console.log(code);
              return {
                content: code,
                moduleType: "svelte",
              };
            } else {
              if (query.svelte && query.type === "style") {
                const css = cache.getCSS(svelteRequest);

                if (css) {
                  return {
                    content: css,
                    moduleType: "css",
                  };
                }
              }
            }
          }
        },
      },

      // async load(id, opts) {
      //   const ssr = !!opts?.ssr;
      //   const svelteRequest = requestParser(id, !!ssr);
      //   if (svelteRequest) {
      //     const { filename, query, raw } = svelteRequest;
      //     if (raw) {
      //       const code = await loadRaw(svelteRequest, compileSvelte, options);
      //       // prevent vite from injecting sourcemaps in the results.
      //       return {
      //         code,
      //         map: {
      //           mappings: "",
      //         },
      //       };
      //     } else {
      //       if (query.svelte && query.type === "style") {
      //         const css = cache.getCSS(svelteRequest);
      //         if (css) {
      //           return css;
      //         }
      //       }
      //       // prevent vite asset plugin from loading files as url that should be compiled in transform
      //       if (viteConfig.assetsInclude(filename)) {
      //         log.debug(
      //           `load returns raw content for ${filename}`,
      //           undefined,
      //           "load"
      //         );
      //         return fs.readFileSync(filename, "utf-8");
      //       }
      //     }
      //   }
      // },

      resolve: {
        filters: {
          sources: [".*"],
          importers: [".*"],
        },
        executor: async (param, context, hookContext) => {
          const isSsr = options.ssr ?? false;
          const svelteRequest = requestParser(param.source, !!options.ssr);
          if (svelteRequest?.query.svelte) {
            if (
              svelteRequest.query.type === "style" &&
              !svelteRequest.raw &&
              !svelteRequest.query.inline
            ) {
              return svelteRequest.cssId;
            }
          }
        },
      },

      // async resolveId(importee, importer, opts) {
      //   const ssr = !!opts?.ssr;
      //   const svelteRequest = requestParser(importee, ssr);
      //   if (svelteRequest?.query.svelte) {
      //     if (
      //       svelteRequest.query.type === "style" &&
      //       !svelteRequest.raw &&
      //       !svelteRequest.query.inline
      //     ) {
      //       // return cssId with root prefix so postcss pipeline of vite finds the directory correctly
      //       // see https://github.com/sveltejs/vite-plugin-svelte/issues/14
      //       log.debug(
      //         `resolveId resolved virtual css module ${svelteRequest.cssId}`,
      //         undefined,
      //         "resolve"
      //       );
      //       return svelteRequest.cssId;
      //     }
      //   }
      // },

      transform: {
        filters: {
          moduleTypes: ["*"],
          resolvedPaths: [".*"],
        },
        async executor(param, ctx) {
          console.log(param);
          // const { css: compiledCss, map } = compileSass(param.content);
          // return {
          //   content: compiledCss,
          //   moduleType: 'css' // transformed sass to css,
          //   sourceMap: JSON.stringify(map)
          //   ignorePreviousSourceMap: false,
          // }
        },
      },

      // async transform(code, id, opts) {
      //   const ssr = !!opts?.ssr;
      //   const svelteRequest = requestParser(id, ssr);
      //   if (
      //     !svelteRequest ||
      //     svelteRequest.query.type === "style" ||
      //     svelteRequest.raw
      //   ) {
      //     return;
      //   }
      //   let compileData;
      //   try {
      //     compileData = await compileSvelte(svelteRequest, code, options);
      //   } catch (e) {
      //     cache.setError(svelteRequest, e);
      //     throw toRollupError(e, options);
      //   }
      //   logCompilerWarnings(
      //     svelteRequest,
      //     compileData.compiled.warnings,
      //     options
      //   );
      //   cache.update(compileData);
      //   if (compileData.dependencies?.length) {
      //     if (options.server) {
      //       for (const dep of compileData.dependencies) {
      //         ensureWatchedFile(options.server.watcher, dep, options.root);
      //       }
      //     } else if (options.isBuild && viteConfig.build.watch) {
      //       for (const dep of compileData.dependencies) {
      //         this.addWatchFile(dep);
      //       }
      //     }
      //   }
      //   return {
      //     ...compileData.compiled.js,
      //     meta: {
      //       vite: {
      //         lang: compileData.lang,
      //       },
      //     },
      //   };
      // },

      // handleHotUpdate(ctx) {
      //   if (!options.compilerOptions.hmr || !options.emitCss) {
      //     return;
      //   }
      //   const svelteRequest = requestParser(ctx.file, false, ctx.timestamp);
      //   if (svelteRequest) {
      //     return handleHotUpdate(
      //       compileSvelte,
      //       ctx,
      //       svelteRequest,
      //       cache,
      //       options
      //     );
      //   }
      // },

      finish: {
        async executor() {
          await options.stats?.finishAll();
        },
      },
    },
    {
      name: "vite-plugin-svelte-module",
      priority: 99,
      // async configResolved() {
      //   moduleRequestParser = buildModuleIdParser(options);
      // },
      // async transform(code, id, opts) {
      //   const ssr = !!opts?.ssr;
      //   const moduleRequest = moduleRequestParser(id, ssr);
      //   if (!moduleRequest) {
      //     return;
      //   }
      //   try {
      //     const compileResult = svelteCompiler.compileModule(code, {
      //       dev: !viteConfig.isProduction,
      //       generate: ssr ? "server" : "client",
      //       filename: moduleRequest.filename,
      //     });
      //     logCompilerWarnings(moduleRequest, compileResult.warnings, options);
      //     return compileResult.js;
      //   } catch (e) {
      //     throw toRollupError(e, options);
      //   }
      // },
    },
  ];
  return plugins;
}

export { vitePreprocess } from "./preprocess.js";
export { loadSvelteConfig } from "./utils/load-svelte-config.js";
