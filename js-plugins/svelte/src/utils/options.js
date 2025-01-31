import process from "node:process";
import {
  defaultClientMainFields,
  defaultServerMainFields,
  defaultClientConditions,
  defaultServerConditions,
  normalizePath,
} from "vite";
import { isDebugNamespaceEnabled, log } from "./log.js";
import { loadSvelteConfig } from "./load-svelte-config.js";
import {
  DEFAULT_SVELTE_EXT,
  FAQ_LINK_MISSING_EXPORTS_CONDITION,
  SVELTE_EXPORT_CONDITIONS,
  SVELTE_IMPORTS,
} from "./constants.js";

import path from "node:path";
import {
  esbuildSvelteModulePlugin,
  esbuildSveltePlugin,
  facadeEsbuildSvelteModulePluginName,
  facadeEsbuildSveltePluginName,
} from "./esbuild.js";
import { addExtraPreprocessors } from "./preprocess.js";
import deepmerge from "deepmerge";
import {
  crawlFrameworkPkgs,
  isDepExcluded,
  isDepExternaled,
  isDepIncluded,
  isDepNoExternaled,
} from "vitefu";

import { isCommonDepWithoutSvelteField } from "./dependencies.js";
import { VitePluginSvelteStats } from "./vite-plugin-svelte-stats.js";

const allowedPluginOptions = new Set([
  "include",
  "exclude",
  "emitCss",
  "hot",
  "ignorePluginPreprocessors",
  "disableDependencyReinclusion",
  "prebundleSvelteLibraries",
  "inspector",
  "dynamicCompileOptions",
  "experimental",
]);

const knownRootOptions = new Set([
  "extensions",
  "compilerOptions",
  "preprocess",
  "onwarn",
]);

const allowedInlineOptions = new Set([
  "configFile",
  ...allowedPluginOptions,
  ...knownRootOptions,
]);

/**
 * @param {Partial<import('../public.d.ts').Options>} [inlineOptions]
 */
export function validateInlineOptions(inlineOptions) {
  const invalidKeys = Object.keys(inlineOptions || {}).filter(
    (key) => !allowedInlineOptions.has(key)
  );
  if (invalidKeys.length) {
    log.warn(
      `invalid plugin options "${invalidKeys.join(", ")}" in inline config`,
      inlineOptions
    );
  }
}

/**
 * @param {Partial<import('../public.d.ts').SvelteConfig>} [config]
 * @returns {Partial<import('../public.d.ts').Options> | undefined}
 */
function convertPluginOptions(config) {
  if (!config) {
    return;
  }
  const invalidRootOptions = Object.keys(config).filter((key) =>
    allowedPluginOptions.has(key)
  );
  if (invalidRootOptions.length > 0) {
    throw new Error(
      `Invalid options in svelte config. Move the following options into 'vitePlugin:{...}': ${invalidRootOptions.join(
        ", "
      )}`
    );
  }
  if (!config.vitePlugin) {
    return config;
  }
  const pluginOptions = config.vitePlugin;
  const pluginOptionKeys = Object.keys(pluginOptions);

  const rootOptionsInPluginOptions = pluginOptionKeys.filter((key) =>
    knownRootOptions.has(key)
  );
  if (rootOptionsInPluginOptions.length > 0) {
    throw new Error(
      `Invalid options in svelte config under vitePlugin:{...}', move them to the config root : ${rootOptionsInPluginOptions.join(
        ", "
      )}`
    );
  }
  const duplicateOptions = pluginOptionKeys.filter((key) =>
    Object.prototype.hasOwnProperty.call(config, key)
  );
  if (duplicateOptions.length > 0) {
    throw new Error(
      `Invalid duplicate options in svelte config under vitePlugin:{...}', they are defined in root too and must only exist once: ${duplicateOptions.join(
        ", "
      )}`
    );
  }
  const unknownPluginOptions = pluginOptionKeys.filter(
    (key) => !allowedPluginOptions.has(key)
  );
  if (unknownPluginOptions.length > 0) {
    log.warn(
      `ignoring unknown plugin options in svelte config under vitePlugin:{...}: ${unknownPluginOptions.join(
        ", "
      )}`
    );
    unknownPluginOptions.forEach((unkownOption) => {
      // @ts-expect-error not typed
      delete pluginOptions[unkownOption];
    });
  }
  /** @type {import('../public.d.ts').Options} */
  const result = {
    ...config,
    ...pluginOptions,
  };
  // @ts-expect-error it exists
  delete result.vitePlugin;

  return result;
}

/**
 * used in config phase, merges the default options, svelte config, and inline options
 * @param {Partial<import('../public.d.ts').Options> | undefined} inlineOptions
 * @param {import('vite').UserConfig} viteUserConfig
 * @param {import('vite').ConfigEnv} viteEnv
 * @returns {Promise<import('../types/options.d.ts').PreResolvedOptions>}
 */
export async function preResolveOptions(
  inlineOptions,
  viteUserConfig,
  viteEnv
) {
  if (!inlineOptions) {
    inlineOptions = {};
  }
  /** @type {import('vite').UserConfig} */
  const viteConfigWithResolvedRoot = {
    ...viteUserConfig,
    root: resolveViteRoot(viteUserConfig),
  };
  const isBuild = viteEnv.command === "build";
  /** @type {Partial<import('../types/options.d.ts').PreResolvedOptions>} */
  const defaultOptions = {
    extensions: DEFAULT_SVELTE_EXT,
    emitCss: true,
    prebundleSvelteLibraries: !isBuild,
  };
  const svelteConfig = convertPluginOptions(
    await loadSvelteConfig(viteConfigWithResolvedRoot, inlineOptions)
  );
  /** @type {Partial<import('../types/options.d.ts').PreResolvedOptions>} */
  const extraOptions = {
    root: viteConfigWithResolvedRoot.root,
    isBuild,
    isServe: viteEnv.command === "serve",
    isDebug: process.env.DEBUG != null,
  };

  const merged =
    /** @type {import('../types/options.d.ts').PreResolvedOptions} */ (
      mergeConfigs(defaultOptions, svelteConfig, inlineOptions, extraOptions)
    );
  // configFile of svelteConfig contains the absolute path it was loaded from,
  // prefer it over the possibly relative inline path
  if (svelteConfig?.configFile) {
    merged.configFile = svelteConfig.configFile;
  }
  return merged;
}

/**
 * @template T
 * @param  {(Partial<T> | undefined)[]} configs
 * @returns T
 */
function mergeConfigs(...configs) {
  /** @type {Partial<T>} */
  let result = {};
  for (const config of configs.filter((x) => x != null)) {
    result = deepmerge(result, /** @type {Partial<T>} */ (config), {
      // replace arrays
      arrayMerge: (target, source) => source ?? target,
    });
  }
  return /** @type {T} */ result;
}

/**
 * used in configResolved phase, merges a contextual default config, pre-resolved options, and some preprocessors. also validates the final config.
 *
 * @param {import('../types/options.d.ts').PreResolvedOptions} preResolveOptions
 * @param {import('vite').ResolvedConfig} viteConfig
 * @param {import('./vite-plugin-svelte-cache.js').VitePluginSvelteCache} cache
 * @returns {import('../types/options.d.ts').ResolvedOptions}
 */
export function resolveOptions(preResolveOptions, viteConfig, cache) {
  const css = preResolveOptions.emitCss ? "external" : "injected";
  /** @type {Partial<import('../public.d.ts').Options>} */
  const defaultOptions = {
    compilerOptions: {
      css,
      dev: !viteConfig.isProduction,
      hmr:
        !viteConfig.isProduction &&
        !preResolveOptions.isBuild &&
        viteConfig.server &&
        viteConfig.server.hmr !== false,
    },
  };

  /** @type {Partial<import('../types/options.d.ts').ResolvedOptions>} */
  const extraOptions = {
    root: viteConfig.root,
    isProduction: viteConfig.isProduction,
  };
  const merged = /** @type {import('../types/options.d.ts').ResolvedOptions}*/ (
    mergeConfigs(defaultOptions, preResolveOptions, extraOptions)
  );

  removeIgnoredOptions(merged);
  handleDeprecatedOptions(merged);
  addExtraPreprocessors(merged, viteConfig);
  enforceOptionsForHmr(merged, viteConfig);
  enforceOptionsForProduction(merged);
  // mergeConfigs would mangle functions on the stats class, so do this afterwards
  if (log.debug.enabled && isDebugNamespaceEnabled("stats")) {
    merged.stats = new VitePluginSvelteStats(cache);
  }
  return merged;
}

/**
 * @param {import('../types/options.d.ts').ResolvedOptions} options
 * @param {import('vite').ResolvedConfig} viteConfig
 */
function enforceOptionsForHmr(options, viteConfig) {
  if (options.hot) {
    log.warn(
      "svelte 5 has hmr integrated in core. Please remove the vitePlugin.hot option and use compilerOptions.hmr instead"
    );
    delete options.hot;
    options.compilerOptions.hmr = true;
  }
  if (options.compilerOptions.hmr && viteConfig.server?.hmr === false) {
    log.warn(
      "vite config server.hmr is false but compilerOptions.hmr is true. Forcing compilerOptions.hmr to false as it would not work."
    );
    options.compilerOptions.hmr = false;
  }
}

/**
 * @param {import('../types/options.d.ts').ResolvedOptions} options
 */
function enforceOptionsForProduction(options) {
  if (options.isProduction) {
    if (options.compilerOptions.hmr) {
      log.warn(
        "you are building for production but compilerOptions.hmr is true, forcing it to false"
      );
      options.compilerOptions.hmr = false;
    }
    if (options.compilerOptions.dev) {
      log.warn(
        "you are building for production but compilerOptions.dev is true, forcing it to false"
      );
      options.compilerOptions.dev = false;
    }
  }
}

/**
 * @param {import('../types/options.d.ts').ResolvedOptions} options
 */
function removeIgnoredOptions(options) {
  const ignoredCompilerOptions = ["generate", "format", "filename"];
  if (options.compilerOptions.hmr && options.emitCss) {
    ignoredCompilerOptions.push("cssHash");
  }
  const passedCompilerOptions = Object.keys(options.compilerOptions || {});
  const passedIgnored = passedCompilerOptions.filter((o) =>
    ignoredCompilerOptions.includes(o)
  );
  if (passedIgnored.length) {
    log.warn(
      `The following Svelte compilerOptions are controlled by vite-plugin-svelte and essential to its functionality. User-specified values are ignored. Please remove them from your configuration: ${passedIgnored.join(
        ", "
      )}`
    );
    passedIgnored.forEach((ignored) => {
      // @ts-expect-error string access
      delete options.compilerOptions[ignored];
    });
  }
}

/**
 * @param {import('../types/options.d.ts').ResolvedOptions} options
 */
function handleDeprecatedOptions(options) {
  const experimental = /** @type {Record<string, any>} */ (
    options.experimental
  );
  if (experimental) {
    for (const promoted of [
      "prebundleSvelteLibraries",
      "inspector",
      "dynamicCompileOptions",
    ]) {
      if (experimental[promoted]) {
        //@ts-expect-error untyped assign
        options[promoted] = experimental[promoted];
        delete experimental[promoted];
        log.warn(
          `Option "experimental.${promoted}" is no longer experimental and has moved to "${promoted}". Please update your Svelte or Vite config.`
        );
      }
    }
    if (experimental.generateMissingPreprocessorSourcemaps) {
      log.warn(
        "experimental.generateMissingPreprocessorSourcemaps has been removed."
      );
    }
  }
}

/**
 * vite passes unresolved `root`option to config hook but we need the resolved value, so do it here
 *
 * @see https://github.com/sveltejs/vite-plugin-svelte/issues/113
 * @see https://github.com/vitejs/vite/blob/43c957de8a99bb326afd732c962f42127b0a4d1e/packages/vite/src/node/config.ts#L293
 *
 * @param {import('vite').UserConfig} viteConfig
 * @returns {string | undefined}
 */
function resolveViteRoot(viteConfig) {
  return normalizePath(
    viteConfig.root ? path.resolve(viteConfig.root) : process.cwd()
  );
}

export async function buildExtraFarmConfig() {
  const extraFarmConfig = {
    resolve: {
      dedupe: [...SVELTE_IMPORTS],
      mainFields: ["svelte"],
      conditions: ["svelte"],
    },
  };
  return {
    compilation: {
      ...extraFarmConfig,
    },
  };
}

/**
 * @param {import('vite').ResolvedConfig} viteConfig
 * @param {import('../types/options.d.ts').ResolvedOptions} options
 */
export function patchResolvedViteConfig(viteConfig, options) {
  if (options.preprocess) {
    for (const preprocessor of arraify(options.preprocess)) {
      if (preprocessor.style && "__resolvedConfig" in preprocessor.style) {
        preprocessor.style.__resolvedConfig = viteConfig;
      }
    }
  }

  // replace facade esbuild plugin with a real one
  const facadeEsbuildSveltePlugin =
    viteConfig.optimizeDeps.esbuildOptions?.plugins?.find(
      (plugin) => plugin.name === facadeEsbuildSveltePluginName
    );
  if (facadeEsbuildSveltePlugin) {
    Object.assign(facadeEsbuildSveltePlugin, esbuildSveltePlugin(options));
  }
  const facadeEsbuildSvelteModulePlugin =
    viteConfig.optimizeDeps.esbuildOptions?.plugins?.find(
      (plugin) => plugin.name === facadeEsbuildSvelteModulePluginName
    );
  if (facadeEsbuildSvelteModulePlugin) {
    Object.assign(
      facadeEsbuildSvelteModulePlugin,
      esbuildSvelteModulePlugin(options)
    );
  }
}

/**
 * Mutates `config` to ensure `resolve.mainFields` is set. If unset, it emulates Vite's default fallback.
 * @param {string} name
 * @param {import('vite').EnvironmentOptions} config
 * @param {{ isSsrTargetWebworker?: boolean }} opts
 */
export function ensureConfigEnvironmentMainFields(name, config, opts) {
  config.resolve ??= {};
  if (config.resolve.mainFields == null) {
    if (
      config.consumer === "client" ||
      name === "client" ||
      opts.isSsrTargetWebworker
    ) {
      config.resolve.mainFields = [...defaultClientMainFields];
    } else {
      config.resolve.mainFields = [...defaultServerMainFields];
    }
  }
  return true;
}

/**
 * Mutates `config` to ensure `resolve.conditions` is set. If unset, it emulates Vite's default fallback.
 * @param {string} name
 * @param {import('vite').EnvironmentOptions} config
 * @param {{ isSsrTargetWebworker?: boolean }} opts
 */
export function ensureConfigEnvironmentConditions(name, config, opts) {
  config.resolve ??= {};
  if (config.resolve.conditions == null) {
    if (
      config.consumer === "client" ||
      name === "client" ||
      opts.isSsrTargetWebworker
    ) {
      config.resolve.conditions = [...defaultClientConditions];
    } else {
      config.resolve.conditions = [...defaultServerConditions];
    }
  }
}

/**
 * @template T
 * @param {T | T[]} value
 * @returns {T[]}
 */
function arraify(value) {
  return Array.isArray(value) ? value : [value];
}
