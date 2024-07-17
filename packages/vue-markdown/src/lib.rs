#![deny(clippy::all)]

mod core;
use core::markdown::create_markdown;
use core::options::Options;
use lazy_static::lazy_static;

lazy_static! {
  static ref MARKDOWN_MODULE_TYPE: String = String::from("markdown-mdx");
}

use farmfe_core::{
  config::Config,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult, PluginTransformHookResult},
};

use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::fs::read_file_utf8;

#[farm_plugin]
pub struct VueMarkdown {
  // options: Options,
}

impl VueMarkdown {
  fn new(_config: &Config, _options: String) -> Self {
    Self {
      // ..Default::default()
    }
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
    if param.resolved_path.ends_with(".mdx") || param.resolved_path.ends_with(".md") {
      let content = read_file_utf8(param.resolved_path).unwrap();
      return Ok(Some(PluginLoadHookResult {
        content,
        source_map: None,
        module_type: ModuleType::Custom(MARKDOWN_MODULE_TYPE.to_string()),
      }));
    }
    Ok(None)
  }

  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    if param.module_type != ModuleType::Custom(MARKDOWN_MODULE_TYPE.to_string()) {
      return Ok(None);
    }

    let transformed_content = create_markdown(
      param.content.clone(),
      Options {
        vue_version: Some("2.0".to_string()),
        wrapper_class: Some("markdown-body".to_string()),
        head_enabled: Some(true),
      },
      param.resolved_path.to_string(),
      param.module_id.clone(),
    );

    return Ok(Some(PluginTransformHookResult {
      content: format!("{}", transformed_content),
      source_map: None,
      module_type: Some(ModuleType::Js),
      ignore_previous_source_map: false,
    }));
  }
}
