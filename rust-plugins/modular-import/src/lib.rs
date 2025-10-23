#![deny(clippy::all)]

use farmfe_core::{
  config::Config,
  error::Result as HookResult,
  plugin::{Plugin, PluginTransformHookParam, PluginTransformHookResult},
  serde_json,
};
use farmfe_macro_plugin::farm_plugin;

mod default;
mod options;
mod transform;

use options::Options;
use transform::transform;

#[farm_plugin]
pub struct FarmfePluginComponent {
  options: Options,
}

impl FarmfePluginComponent {
  fn new(_config: &Config, options: String) -> Self {
    let options: Options = serde_json::from_str(&options).unwrap();
    // println!("Parsed options: {:?}", options); // 添加调试信息
    Self { options }
  }
}

impl Plugin for FarmfePluginComponent {
  fn name(&self) -> &str {
    "FarmfePluginComponent"
  }

  fn transform(
    &self,
    param: &PluginTransformHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> HookResult<Option<PluginTransformHookResult>> {
    transform(&self.options, param)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_plugin_name() {
    let config = Config::default();
    let options = r#"{}"#.to_string();
    let plugin = FarmfePluginComponent::new(&config, options);
    assert_eq!(plugin.name(), "FarmfePluginComponent");
  }

  #[test]
  fn test_options_deserialization() {
    let json = r#"{}"#;
    let options: Options = serde_json::from_str(json).unwrap();
    // Just verify it can be deserialized
    drop(options);
  }
}
