#![deny(clippy::all)]

mod options;
mod plugin_component;
mod utils;
mod core;

use farmfe_core::{
  config::Config,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult},
};

use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::fs::read_file_utf8;

#[farm_plugin]
pub struct VueMarkdown {}

impl VueMarkdown {
  fn new(config: &Config, options: String) -> Self {
    Self {}
  }
}

impl Plugin for VueMarkdown {
  fn name(&self) -> &str {
    "VueMarkdown"
  }

  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    if param.resolved_path.ends_with((".mdx")) || param.resolved_path.ends_with(".md") {
      let content = read_file_utf8(param.resolved_path).unwrap();

      return Ok(Some(PluginLoadHookResult {
        content,
        source_map: None,
        module_type: ModuleType::Custom("vue-markdown".to_string()),
      }));
    }
    Ok(None)
  }

  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    if param.module_type != ModuleType::Custom("vue-markdown".to_string()) {
      return Ok(None);
    }
  }
}
