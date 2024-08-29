#![deny(clippy::all)]

mod parser;
use std::path::Path;

use farmfe_core::{
  config::Config,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult},
  serde_json,
};
use parser::remix_parser::{build_routes_virtual_code, get_route_files, parse};

use farmfe_macro_plugin::farm_plugin;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
enum Mode {
  Remix,
  Next,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Options {
  mode: Mode,
  routes_path: String,
  emit_file: Option<String>,
}

#[farm_plugin]
pub struct FarmPluginReactRouter {
  options: Options,
}

impl FarmPluginReactRouter {
  fn new(_config: &Config, options: String) -> Self {
    let options: Options = serde_json::from_str(&options).unwrap();
    Self { options }
  }
}

impl Plugin for FarmPluginReactRouter {
  fn name(&self) -> &str {
    "FarmPluginReactRouter"
  }
  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    if param.module_id == "virtual:routes" {
      let route_files = get_route_files(Path::new(&param.resolved_path));
      let (routes, imports) = parse(route_files, &param.resolved_path, 0);
      let code = build_routes_virtual_code(routes, imports);
      return Ok(Some(PluginLoadHookResult {
        content: code,
        module_type: ModuleType::Js,
        source_map: None,
      }));
    }
    Ok(None)
  }
}
