#![deny(clippy::all)]
mod cache;
use std::{collections::HashMap, path::Path};

use base64::{engine::general_purpose, Engine};
use farmfe_compiler::Compiler;
use farmfe_core::{
  config::{
    bool_or_obj,
    config_regex::ConfigRegex,
    partial_bundling::{PartialBundlingConfig, PartialBundlingEnforceResourceConfig},
    Config, Mode, ModuleFormat, OutputConfig, TargetEnv,
  },
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult, PluginResolveHookResult, PluginTransformHookResult},
  relative_path::RelativePath,
  serde, serde_json,
};
use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::fs::transform_output_filename;
use farmfe_utils::parse_query;
use regress::Regex as JsRegex;
use serde::{Deserialize, Serialize};

const WORKER_OR_SHARED_WORKER_RE: &str = r#"(?:\?|&)(worker|sharedworker)(?:&|$)"#;
const WORKER_FILE_RE: &str = r#"(?:\?|&)worker_file&type=(\w+)(?:&|$)"#;
const INLINE_RE: &str = r#"[?&]inline\b"#;
const WORKER_FILE_ID: &str = "worker_file";
fn get_worker_cache_dir(root: &str) -> String {
  RelativePath::new("node_modules/.farm/cache/workers")
    .to_logical_path(root)
    .to_string_lossy()
    .to_string()
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
    let (clean_path, query) = &param.source.split_once("?").unwrap();
    let query = parse_query(&format!("?{}", query));
    if JsRegex::new(WORKER_OR_SHARED_WORKER_RE)
      .unwrap()
      .find(id)
      .is_some()
      || query.iter().any(|(k, _v)| k == WORKER_FILE_ID)
    {
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
      || param.query.iter().any(|(k, _v)| k == WORKER_FILE_ID)
    // query has WORKER_FILE_ID
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
    println!("is_build: {is_build}");
    let compiler_config = self.options.compiler_config.clone().unwrap();
    let worker_file_match = JsRegex::new(WORKER_FILE_RE).unwrap().find(&param.module_id);
    if worker_file_match.is_some() {
      let worker_type = &param.module_id[worker_file_match.unwrap().group(1).unwrap()];
      // worker file path
      let script_path = context.config.root.clone();
      let inject_env = match worker_type {
        "classic" => format!("importScripts('{}')", script_path.to_string()),
        "module" => format!("import {}\n", script_path.to_string()),
        "ignore" => String::new(),
        _ => String::new(),
      };
      println!("[FarmfePluginWorker] inject_env: {:?}", inject_env);
      if !inject_env.is_empty() {
        return Ok(Some(PluginTransformHookResult {
          content: format!(";/n{}", inject_env),
          module_type: Some(ModuleType::Js),
          source_map: None,
          ..Default::default()
        }));
      } else {
        return Ok(None);
      }
    }

    let worker_match = JsRegex::new(WORKER_OR_SHARED_WORKER_RE)
      .unwrap()
      .find(&param.module_id);
    if worker_match.is_none() {
      return Ok(None);
    }

    let worker_constructor = &param.module_id[worker_match.unwrap().group(1).unwrap()];

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
        let file_name_ext = Path::new(&param.resolved_path)
          .file_name()
          .map(|x| x.to_string_lossy().to_string())
          .unwrap();
        let (file_name, ext) = file_name_ext.split_once(".").unwrap();
        let assets_filename_config = compiler_config.output.assets_filename.clone();
        let full_file_name = transform_output_filename(
          assets_filename_config,
          &file_name,
          file_name.as_bytes(),
          ext,
        );
        let mut input = HashMap::new();
        input.insert(full_file_name.clone(), param.resolved_path.to_string());
        let compiler = Compiler::new(
          Config {
            input,
            partial_bundling: Box::new(PartialBundlingConfig {
              enforce_resources: vec![PartialBundlingEnforceResourceConfig {
                name: file_name.to_string(),
                test: vec![ConfigRegex::new(".+")],
              }],
              ..*compiler_config.partial_bundling.clone()
            }),
            output: Box::new(OutputConfig {
              target_env: TargetEnv::Custom("library-browser".to_string()),
              ..*compiler_config.output.clone()
            }),
            minify: Box::new(bool_or_obj::BoolOrObj::Bool(false)),
            ..compiler_config
          },
          vec![],
        )
        .unwrap();
        compiler.compile().unwrap();
        let resources_map = compiler.context().resources_map.lock();
        let resource_name = format!("{}.js", full_file_name);
        let resource = resources_map.get(&resource_name).unwrap();
        let content_bytes = resource.bytes.clone();
        // let content = String::from_utf8_lossy(&content_bytes).to_string();
        // println!("content: {:?}", content);
        // self
        //   .worker_cache
        //   .insert(resource_name.clone(), content_bytes.clone());
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
        println!("is build code: {}", code);
        return Ok(Some(PluginTransformHookResult {
          content: code,
          module_type: Some(ModuleType::Js),
          source_map: None,
          ..Default::default()
        }));
      }
    } else {
      let clean_url = param.module_id.split_once("?").unwrap().0;
      let url = format!("{}?{}&type={}", clean_url, WORKER_FILE_ID, worker_type);
      url_code = url;
    }
    if param.query.iter().any(|(k, _v)| k == "url") {
      return Ok(Some(PluginTransformHookResult {
        content: format!("export default {}", url_code),
        module_type: Some(ModuleType::Js),
        source_map: None,
        ..Default::default()
      }));
    }
    println!("url_code: {}", url_code);
    return Ok(Some(PluginTransformHookResult {
      content: format!(
        r#"
        export default function WorkerWrapper(options) {{
          return new {0}(
            "{1}",
            {2}
          );
        }}"#,
        worker_constructor, url_code, worker_type_option
      ),
      module_type: Some(ModuleType::Js),
      ..Default::default()
    }));
  }
}
