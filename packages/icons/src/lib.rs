#![deny(clippy::all)]
mod options;
mod svg_id;

use farmfe_core::{config::Config, plugin::Plugin, serde_json};
use farmfe_macro_plugin::farm_plugin;
use options::Options;

#[farm_plugin]
pub struct FarmfePluginIcons {
  options: Options,
}

impl FarmfePluginIcons {
  fn new(config: &Config, _options: String) -> Self {
    let options: Options = serde_json::from_str(&_options).unwrap();
    let collections_node_resolve_path = Some(
      options
        .collections_node_resolve_path
        .unwrap_or(config.root.clone()),
    );
    Self {
      options: Options {
        collections_node_resolve_path,
        ..options
      },
    }
  }
}

impl Plugin for FarmfePluginIcons {
  fn name(&self) -> &str {
    "FarmfePluginIcons"
  }
  fn load(
    &self,
    _param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    Ok(None)
  }
}
