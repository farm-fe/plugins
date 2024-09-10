#![deny(clippy::all)]

mod parser;
use farmfe_core::{
  config::Config,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult, PluginResolveHookResult},
  serde_json,
};
use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::pluginutils::normalize_path::normalize_path;
use parser::remix_parser::{build_routes_virtual_code, get_route_files, parse};
use serde::{Deserialize, Serialize};

const REACT_VIRTUAL_ROUTER: &str = "virtual:__REACT_VIRTUAL_ROUTER__";

#[derive(Debug, Deserialize, Serialize, Clone)]
enum Mode {
  Remix,
  Next,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Options {
  mode: Option<Mode>,
  routes_path: Option<String>,
}

#[farm_plugin]
pub struct FarmPluginReactRouter {
  options: Options,
}

impl FarmPluginReactRouter {
  fn new(config: &Config, options: String) -> Self {
    let root_path = config.root.clone();
    let default_routes_path = normalize_path(&format!("{}/src/routes", root_path));
    let options: Options = serde_json::from_str(&options).unwrap();
    let options = Options {
      mode: Some(options.mode.unwrap_or(Mode::Remix)),
      routes_path: Some(options.routes_path.unwrap_or(default_routes_path)),
    };
    Self { options }
  }
}

impl Plugin for FarmPluginReactRouter {
  fn name(&self) -> &str {
    "FarmPluginReactRouter"
  }
  fn priority(&self) -> i32 {
    105
  }
  fn resolve(
    &self,
    param: &farmfe_core::plugin::PluginResolveHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginResolveHookResult>> {
    if param.source == "virtual:react-routes" {
      return Ok(Some(PluginResolveHookResult {
        resolved_path: REACT_VIRTUAL_ROUTER.to_string(),
        side_effects: false,
        external: false,
        ..Default::default()
      }));
    }
    Ok(None)
  }
  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    if param.resolved_path == REACT_VIRTUAL_ROUTER {
      if matches!(self.options.mode, Some(Mode::Remix)) {
        let route_files = get_route_files(&self.options.routes_path.clone().unwrap());
        let (routes, imports) = parse(route_files, &self.options.routes_path.clone().unwrap(), 0);
        let code = build_routes_virtual_code(routes, imports);
        return Ok(Some(PluginLoadHookResult {
          content: code,
          module_type: ModuleType::Js,
          source_map: None,
        }));
      }
    }
    Ok(None)
  }
}
