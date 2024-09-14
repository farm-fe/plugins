#![deny(clippy::all)]
use farmfe_core::module::ModuleType;
use mdxjs::compile;

use farmfe_core::{config::Config, plugin::Plugin};

use farmfe_core::plugin::PluginLoadHookResult;
use farmfe_macro_plugin::farm_plugin;
use std::fs::read_to_string;

#[farm_plugin]
pub struct FarmPluginMdx {}

fn is_mdx_file(file_name: &String) -> bool {
  file_name.ends_with(".md") || file_name.ends_with(".mdx")
}

impl FarmPluginMdx {
  fn new(_config: &Config, _options: String) -> Self {
    Self {}
  }
}

impl Plugin for FarmPluginMdx {
  fn name(&self) -> &str {
    "FarmPluginMdx"
  }

  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<PluginLoadHookResult>> {
    if is_mdx_file(&param.module_id) {
      let content = read_to_string(param.resolved_path).unwrap();
      return Ok(Some(farmfe_core::plugin::PluginLoadHookResult {
        content,
        source_map: None,
        module_type: ModuleType::Custom("mdx".to_string()),
      }));
    }
    Ok(None)
  }

  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    if param.module_type != ModuleType::Custom("mdx".to_string()) {
      return Ok(None);
    }
    if param.module_id.ends_with(".mdx") || param.module_id.ends_with(".md") {
      let code = compile(&param.content, &Default::default());
      let js_code = code.unwrap();
      return Ok(Some(farmfe_core::plugin::PluginTransformHookResult {
        content: js_code.to_string(),
        module_type: Some(ModuleType::Jsx),
        source_map: None,
        ignore_previous_source_map: true,
      }));
    }
    println!("{}", param.module_id);
    return Ok(None);
  }
}
