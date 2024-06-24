#![deny(clippy::all)]

use farmfe_core::{config::Config, module::ModuleType, plugin::Plugin, serde_json};
use std::fs::read_to_string;

use farmfe_macro_plugin::farm_plugin;

#[farm_plugin]
pub struct FarmPluginYaml {
  // documentMode: String,
  // extensions: Vec<String>,
}

impl FarmPluginYaml {
  fn new(_config: &Config, _options: String) -> Self {
    Self {
      // documentMode: "single".to_string(),
      // extensions: vec!["yaml".to_string()],
    }
  }
}

impl Plugin for FarmPluginYaml {
  fn name(&self) -> &str {
    "FarmPluginYaml"
  }
  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    if param.module_id.ends_with(".yaml") || param.module_id.ends_with(".yml") {
      let content = read_to_string(param.resolved_path).unwrap();
      return Ok(Some(farmfe_core::plugin::PluginLoadHookResult {
        content,
        source_map: None,
        module_type: ModuleType::Custom(String::from("yaml")),
      }));
    }
    Ok(None)
  }
  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    if param.module_type != ModuleType::Custom(String::from("yaml")) {
      return Ok(None);
    }
    let content = param.content.clone();
    let result: serde_json::Value = serde_yaml::from_str::<serde_json::Value>(&content).unwrap();
    let mut export_val = String::new();

    if let serde_json::Value::Object(object) = result.clone() {
      for (key, val) in object {
        export_val.push_str(&format!("export var {} =  {};\n", key, val));
      }
    }
    return Ok(Some(farmfe_core::plugin::PluginTransformHookResult {
      content: format!("export default {}\n\n {}", result, export_val),
      module_type: Some(ModuleType::Js),
      source_map: None,
      ignore_previous_source_map: false,
    }));
  }
}
