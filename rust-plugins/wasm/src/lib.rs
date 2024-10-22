#![deny(clippy::all)]

use farmfe_core::{
  config::Config,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult, PluginResolveHookResult},
};

use farmfe_macro_plugin::farm_plugin;

const WASM_HELPER_ID_FARM: &str = "\0farm/wasm-helper.js";
const WASM_HELPER_ID_VITE: &str = "\0vite/wasm-helper.js";

#[farm_plugin]
pub struct FarmfePluginWasm {}

impl FarmfePluginWasm {
  fn new(config: &Config, options: String) -> Self {
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
    if let Some(ref importer) = param.importer {
      let id = importer.relative_path();
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
    }

    Ok(None)
  }

  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    if param.resolved_path == WASM_HELPER_ID_FARM || param.resolved_path == WASM_HELPER_ID_VITE {
      return Ok(Some(PluginLoadHookResult {
        content: include_str!("wasm_runtime.js").to_string(),
        module_type: ModuleType::Js,
        source_map: None,
      }));
    }
    let query = &param.query;
    if query.iter().any(|(k, _)| k == "init") {
    }
    Ok(None)
  }
}
