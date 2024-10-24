#![deny(clippy::all)]
mod cache;
use std::{collections::HashMap, path::Path};

use farmfe_compiler::Compiler;
use farmfe_core::{
  config::{
    config_regex::ConfigRegex,
    partial_bundling::{PartialBundlingConfig, PartialBundlingEnforceResourceConfig},
    Config, Mode, OutputConfig, TargetEnv,
  },
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult},
  serde, serde_json,
};
use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::fs::transform_output_filename;
use regress::Regex as JsRegex;
use serde::{Deserialize, Serialize};

const WORKER_OR_SHARED_WORKER_RE: &str = r#"/(?:\?|&)(worker|sharedworker)(?:&|$)/"#;
const WORKER_FILE_RE: &str = r#"/(?:\?|&)worker_file&type=(\w+)(?:&|$)/"#;
const INLINE_RE: &str = r#"/[?&]inline\b/"#;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct Options {
  compiler_config: Config,
}

#[farm_plugin]
pub struct FarmfePluginWorker {
  options: Options,
  worker_cache: cache::WorkerCache,
}

impl FarmfePluginWorker {
  fn new(config: &Config, options: String) -> Self {
    let options: Options = serde_json::from_str(&options).unwrap();
    let worker_cache = cache::WorkerCache::new();
    Self {
      options,
      worker_cache,
    }
  }
}

impl Plugin for FarmfePluginWorker {
  fn name(&self) -> &str {
    "FarmfePluginWorker"
  }

  fn resolve(
    &self,
    param: &farmfe_core::plugin::PluginResolveHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginResolveHookResult>> {
    let id = &param.source;
    if JsRegex::new(WORKER_OR_SHARED_WORKER_RE)
      .unwrap()
      .find(id)
      .is_none()
    {
      return Ok(None);
    }

    return Ok(None);
  }

  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    if JsRegex::new(WORKER_OR_SHARED_WORKER_RE)
      .unwrap()
      .find(&param.module_id)
      .is_some()
    {
      return Ok(Some(PluginLoadHookResult {
        content: String::new(),
        module_type: ModuleType::Custom("worker".to_string()),
        source_map: None,
      }));
    }
    return Ok(None);
  }

  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    if param.module_type != ModuleType::Custom("worker".to_string()) {
      return Ok(None);
    }
    match context.config.mode {
      Mode::Production => {
        let file_name_ext = Path::new(&param.resolved_path)
          .file_name()
          .map(|x| x.to_string_lossy().to_string())
          .unwrap();
        let (file_name, ext) = file_name_ext.split_once(".").unwrap();
        let assets_filename_config = self.options.compiler_config.output.assets_filename.clone();
        let file_name = transform_output_filename(
          assets_filename_config,
          &file_name,
          file_name.as_bytes(),
          ext,
        );
        let mut input = HashMap::new();
        input.insert(file_name.clone(), param.resolved_path.to_string());
        let compiler = Compiler::new(
          Config {
            input,
            partial_bundling: Box::new(PartialBundlingConfig {
              enforce_resources: vec![PartialBundlingEnforceResourceConfig {
                name: file_name,
                test: vec![ConfigRegex::new(".+")],
              }],
              ..*self.options.compiler_config.partial_bundling.clone()
            }),
            output: Box::new(OutputConfig {
              target_env: TargetEnv::Custom("library-browser".to_string()),
              ..*self.options.compiler_config.output.clone()
            }),
            ..self.options.compiler_config.clone()
          },
          vec![],
        )
        .unwrap();
        let bandle = compiler.compile().unwrap();

      }
      Mode::Development => {}
    }
    return Ok(None);
  }
}
