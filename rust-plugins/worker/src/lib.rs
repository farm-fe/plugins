#![deny(clippy::all)]
mod cache;
use std::{collections::HashMap, path::Path};

use base64::{engine::general_purpose, Engine};
use cache::WorkerCache;
use farmfe_compiler::Compiler;
use farmfe_core::{
  config::{
    bool_or_obj,
    config_regex::ConfigRegex,
    partial_bundling::{PartialBundlingConfig, PartialBundlingEnforceResourceConfig},
    Config, ModuleFormat, OutputConfig, TargetEnv,
  },
  context::EmitFileParams,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult, PluginResolveHookResult, PluginTransformHookResult},
  resource::ResourceType,
  serde, serde_json,
};
use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::fs::transform_output_filename;
use farmfe_utils::parse_query;
use regress::Regex as JsRegex;
use serde::{Deserialize, Serialize};

const WORKER_OR_SHARED_WORKER_RE: &str = r#"(?:\?|&)(worker|sharedworker)(?:&|$)"#;
const INLINE_RE: &str = r#"[?&]inline\b"#;

fn build_worker(resolved_path: &str, compiler_config: &Config) -> Vec<u8> {
  let (_worker_url, full_file_name) = get_worker_url(resolved_path, compiler_config);
  let mut input = HashMap::new();
  input.insert(full_file_name.clone(), resolved_path.to_string());
  let compiler = Compiler::new(
    Config {
      input,
      partial_bundling: Box::new(PartialBundlingConfig {
        enforce_resources: vec![PartialBundlingEnforceResourceConfig {
          name: full_file_name.to_string(),
          test: vec![ConfigRegex::new(".+")],
        }],
        ..*compiler_config.partial_bundling.clone()
      }),
      output: Box::new(OutputConfig {
        target_env: TargetEnv::Custom("library-browser".to_string()),
        ..*compiler_config.output.clone()
      }),
      minify: Box::new(bool_or_obj::BoolOrObj::Bool(false)),
      ..compiler_config.clone()
    },
    vec![],
  )
  .unwrap();
  compiler.compile().unwrap();
  let resources_map = compiler.context().resources_map.lock();
  let resource_name = format!("{}.js", full_file_name);
  let resource = resources_map.get(&resource_name).unwrap();
  let content_bytes = resource.bytes.clone();
  content_bytes
}

fn emit_worker_file(
  resolved_path: &str,
  file_name: &str,
  content_bytes: Vec<u8>,
  context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
) {
  let params = EmitFileParams {
    resolved_path: resolved_path.to_string(),
    content: content_bytes,
    name: file_name.to_string(),
    resource_type: ResourceType::Js,
  };
  context.emit_file(params);
}

fn get_worker_url(resolved_path: &str, compiler_config: &Config) -> (String, String) {
  let file_name_ext = Path::new(resolved_path)
    .file_name()
    .map(|x| x.to_string_lossy().to_string())
    .unwrap();
  let (file_name, ext) = file_name_ext.split_once(".").unwrap();
  let assets_filename_config = compiler_config.output.assets_filename.clone();
  let file_name = transform_output_filename(
    assets_filename_config,
    &file_name,
    file_name.as_bytes(),
    ext,
  );
  // worker.ts -> worker.js
  let file_name = if file_name.ends_with(".ts") {
    file_name.replace(".ts", ".js")
  } else {
    file_name
  };
  let worker_url = if !compiler_config.output.public_path.is_empty() {
    let normalized_public_path = compiler_config.output.public_path.trim_end_matches("/");
    format!("{}/{}", normalized_public_path, file_name)
  } else {
    format!("/{}", file_name)
  };
  (worker_url, file_name)
}
struct ProcessWorkerParam<'a> {
  resolved_path: &'a str,
  module_id: &'a str,
  is_build: bool,
  compiler_config: &'a Config,
  worker_cache: &'a WorkerCache,
  is_url: bool,
  context: &'a std::sync::Arc<farmfe_core::context::CompilationContext>,
}

fn process_worker(param: ProcessWorkerParam) -> String {
  let ProcessWorkerParam {
    module_id,
    is_build,
    compiler_config,
    worker_cache,
    resolved_path,
    is_url,
    context,
  } = param;

  let (worker_url, file_name) = get_worker_url(resolved_path, compiler_config);

  let content_bytes = build_worker(resolved_path, &compiler_config);

  if worker_cache.get(resolved_path).is_none() {
    let content_bytes =
      insert_worker_cache(&worker_cache, resolved_path.to_string(), content_bytes);
    emit_worker_file(resolved_path, &file_name, content_bytes, context);
  } else {
    let catch_content_bytes = worker_cache.get(resolved_path).unwrap();
    if content_bytes != catch_content_bytes {
      let content_bytes =
        insert_worker_cache(&worker_cache, resolved_path.to_string(), content_bytes);
      emit_worker_file(resolved_path, &file_name, content_bytes, context);
    }
  }

  let worker_match = JsRegex::new(WORKER_OR_SHARED_WORKER_RE)
    .unwrap()
    .find(&param.module_id);
  let worker_constructor = &module_id[worker_match.unwrap().group(1).unwrap()];

  let worker_constructor = match worker_constructor {
    "sharedworker" => "SharedWorker",
    _ => "Worker",
  };

  let worker_type = if is_build {
    "module"
  } else {
    match &compiler_config.output.format {
      ModuleFormat::EsModule => "module",
      _ => "classic",
    }
  };

  let worker_type_option = match worker_type {
    "module" => "{type: 'module', name: options?.name}",
    _ => "{name: options?.name}",
  };
  let mut url_code = String::new();
  if is_build {
    let worker_inline_match = JsRegex::new(INLINE_RE).unwrap().find(&param.module_id);
    if worker_inline_match.is_some() {
      let content_bytes = worker_cache.get(resolved_path).unwrap();
      let content_base64 = general_purpose::STANDARD.encode(content_bytes);
      let code = if worker_constructor == "Worker" {
        let blob_url = if worker_type == "classic" {
          String::from("")
        } else {
          String::from("'URL.revokeObjectURL(import.meta.url);',")
        };

        format!(
          r#"{0}
            const decodeBase64 = (base64) => Uint8Array.from(atob(base64), c => c.charCodeAt(0));
            const blob = typeof self !== "undefined" && self.Blob && new Blob([{1}decodeBase64(encodedJs)], {{ type: "text/javascript;charset=utf-8" }});
            export default function WorkerWrapper(options) {{
              let objURL;
              try {{
                objURL = blob && (self.URL || self.webkitURL).createObjectURL(blob);
                if (!objURL) throw ''
                const worker = new {2}(objURL, {3});
                worker.addEventListener("error", () => {{
                  (self.URL || self.webkitURL).revokeObjectURL(objURL);
                }});
                return worker;
              }} catch(e) {{
                return new {2}(
                  "data:text/javascript;base64," + encodedJs,
                  {3}
                );
              }}{4}
            }}"#,
          content_base64,
          blob_url,
          worker_constructor,
          worker_type_option,
          if worker_type == "classic" {
            String::from(
              r#" finally {
                      objURL && (self.URL || self.webkitURL).revokeObjectURL(objURL);
                    }"#,
            )
          } else {
            String::from("")
          }
        )
      } else {
        format!(
          r#"{0}
            export default function WorkerWrapper(options) {{
              return new {1}(
                "data:text/javascript;base64," + {0},
                {2}
              );
            }}"#,
          content_base64, worker_constructor, worker_type_option
        )
      };
      return code;
    }
  } else {
    url_code = file_name
  }
  if is_url {
    return format!(r#"export default "{}""#, url_code);
  }
  return format!(
    r#"
      export default function WorkerWrapper(options) {{
        return new {0}(
          "{1}",
          {2}
        );
      }}"#,
    worker_constructor, worker_url, worker_type_option
  );
}

fn insert_worker_cache(worker_cache: &WorkerCache, key: String, content_bytes: Vec<u8>) -> Vec<u8> {
  worker_cache.insert(key.clone(), content_bytes);
  worker_cache.get(&key).unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Options {
  is_build: Option<bool>,
  compiler_config: Option<Config>,
}

#[farm_plugin]
pub struct FarmfePluginWorker {
  options: Options,
  worker_cache: cache::WorkerCache,
}

impl FarmfePluginWorker {
  fn new(_config: &Config, options: String) -> Self {
    let options: Options = serde_json::from_str(&options).unwrap();
    let worker_cache = cache::WorkerCache::new();
    Self {
      options: Options {
        is_build: Some(options.is_build.unwrap_or(false)),
        compiler_config: Some(options.compiler_config.unwrap_or(Config::default())),
      },
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
      .is_some()
    {
      let (clean_path, query) = &param.source.split_once("?").unwrap_or((id, ""));
      let query = parse_query(&format!("?{}", query));
      return Ok(Some(PluginResolveHookResult {
        resolved_path: clean_path.to_string(),
        query,
        ..Default::default()
      }));
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
    let is_build = self.options.is_build.unwrap();
    let compiler_config = self.options.compiler_config.clone().unwrap();

    let worker_match = JsRegex::new(WORKER_OR_SHARED_WORKER_RE)
      .unwrap()
      .find(&param.module_id);
    if worker_match.is_none() {
      return Ok(None);
    }

    let code = process_worker(ProcessWorkerParam {
      resolved_path: param.resolved_path,
      module_id: &param.module_id,
      is_build,
      is_url: param.query.iter().any(|(k, _v)| k == "url"),
      compiler_config: &compiler_config,
      worker_cache: &self.worker_cache,
      context,
    });
    return Ok(Some(PluginTransformHookResult {
      content: code,
      module_type: Some(ModuleType::Js),
      source_map: None,
      ..Default::default()
    }));
  }
}
