#![deny(clippy::all)]

use std::{fs, path::Path};

use farmfe_core::{
  config::Config,
  context::EmitFileParams,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult, PluginResolveHookResult},
  resource::ResourceType,
};
use farmfe_macro_plugin::farm_plugin;

const WASM_HELPER_ID_FARM: &str = "\0farm/wasm-helper.js";
const WASM_HELPER_ID_VITE: &str = "\0vite/wasm-helper.js";

#[farm_plugin]
pub struct FarmfePluginWasm {}

impl FarmfePluginWasm {
  fn new(_config: &Config, _options: String) -> Self {
    Self {}
  }
}

impl Plugin for FarmfePluginWasm {
  fn name(&self) -> &str {
    "FarmfePluginWasm"
  }
  fn resolve(
    &self,
    param: &farmfe_core::plugin::PluginResolveHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginResolveHookResult>> {
    let id = &param.source;
    if id == WASM_HELPER_ID_FARM || id == WASM_HELPER_ID_VITE {
      return Ok(Some(PluginResolveHookResult {
        resolved_path: id.to_string(),
        ..Default::default()
      }));
    }

    if id.ends_with(".wasm?init") {
      return Ok(Some(PluginResolveHookResult {
        resolved_path: id.replace("?init", ""),
        query: vec![("init".to_string(), "".to_string())]
          .into_iter()
          .collect(),
        ..Default::default()
      }));
    }

    // if id.ends_with(".wasm?url") {
    //   return Ok(Some(PluginResolveHookResult {
    //     resolved_path: id.to_string(),
    //     query: vec![("url".to_string(), "".to_string())]
    //       .into_iter()
    //       .collect(),
    //     ..Default::default()
    //   }));
    // }

    Ok(None)
  }

  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    println!("resolved_path: {}", &param.resolved_path);
    if param.resolved_path == WASM_HELPER_ID_FARM || param.resolved_path == WASM_HELPER_ID_VITE {
      return Ok(Some(PluginLoadHookResult {
        content: include_str!("wasm_runtime.js").to_string(),
        module_type: ModuleType::Js,
        source_map: None,
      }));
    }
    let query = &param.query;
    if query.iter().any(|(k, _)| k == "init") {
      let file_name = Path::new(&param.resolved_path)
        .file_name()
        .map(|x| x.to_string_lossy().to_string());
      let content = fs::read(&param.resolved_path).unwrap();
      let params = EmitFileParams {
        name: file_name.clone().unwrap(),
        content,
        resource_type: ResourceType::Asset("wasm".to_string()),
        resolved_path: param.resolved_path.to_string(),
      };
      context.emit_file(params);
      let url = format!("/{}", param.module_id.replace("?init", ""));
      let code = format!(
        r#"import initWasm from "{WASM_HELPER_ID_FARM}"; 
        export default opts => initWasm(opts, "{url}")"#
      );
      return Ok(Some(PluginLoadHookResult {
        content: code,
        module_type: ModuleType::Js,
        source_map: None,
      }));
    }
    Ok(None)
  }
}
