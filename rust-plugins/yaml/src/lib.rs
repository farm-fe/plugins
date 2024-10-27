#![deny(clippy::all)]

use farmfe_core::{config::Config, module::ModuleType, plugin::Plugin, serde_json};
use farmfe_macro_plugin::farm_plugin;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::fs::read_to_string;

lazy_static! {
  static ref YAML_MODULE_TYPE: String = String::from("yaml");
}

fn is_yaml_file(file_name: &String) -> bool {
  file_name.ends_with(".yaml") || file_name.ends_with(".yml")
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum DocumentMode {
  Single,
  Multi
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FarmPluginYamlOptions {
  document_mode: Option<DocumentMode>,
  include: Option<String>,
  exclude: Option<String>,
}

#[farm_plugin]
pub struct FarmPluginYaml {
  document_mode: DocumentMode,
  include: String,
  exclude: String,
}

impl FarmPluginYaml {
  fn new(_config: &Config, options: String) -> Self {
    let yaml_options: FarmPluginYamlOptions = serde_json::from_str(&options).unwrap();
    let include: String = yaml_options.include.unwrap_or(String::from(""));
    let exclude: String = yaml_options.exclude.unwrap_or(String::from(""));
    Self {
      document_mode: yaml_options.document_mode.unwrap_or(DocumentMode::Single),
      include,
      exclude,
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
    if is_yaml_file(&param.module_id) {
      let content = read_to_string(param.resolved_path).unwrap();
      return Ok(Some(farmfe_core::plugin::PluginLoadHookResult {
        content,
        source_map: None,
        module_type: ModuleType::Custom(YAML_MODULE_TYPE.to_string()),
      }));
    }
    Ok(None)
  }
  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    if param.module_type != ModuleType::Custom(YAML_MODULE_TYPE.to_string()) {
      return Ok(None);
    }

    if !self.include.is_empty() {
      let inc_reg = Regex::new(&format!("{}", self.include)).unwrap();
      if let Some(_text) = inc_reg.find(param.resolved_path) {
      } else {
        return Ok(None);
      }
    }

    if !self.exclude.is_empty() {
      let exc_reg = Regex::new(&format!("{}", self.exclude)).unwrap();
      if let Some(_text) = exc_reg.find(param.resolved_path) {
        return Ok(None);
      }
    }

    let code = match self.document_mode {
      DocumentMode::Single => {
        let result: serde_json::Value =
          serde_yaml::from_str::<serde_json::Value>(&param.content).unwrap();
        let mut export_val = String::new();

        if let serde_json::Value::Object(object) = result.clone() {
          for (key, val) in object {
            export_val.push_str(&format!("export var {} =  {};\n", key, val));
          }
        }
        format!("export default {}\n\n {}", result, export_val)
      }
      DocumentMode::Multi => {
        let result: serde_json::Value =
          serde_yaml::from_str::<serde_json::Value>(&param.content).unwrap();
        let mut export_val = String::new();
        if let serde_json::Value::Object(object) = result.clone() {
          for (key, val) in object {
            export_val.push_str(&format!("export var {} =  {};\n", key, val));
          }
        }
        format!("export default {}\n\n {}", result, export_val)
      }
    };

    return Ok(Some(farmfe_core::plugin::PluginTransformHookResult {
      content: code,
      module_type: Some(ModuleType::Js),
      source_map: None,
      ignore_previous_source_map: false,
    }));
  }
}
