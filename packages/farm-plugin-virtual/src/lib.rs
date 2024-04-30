#![deny(clippy::all)]
mod utils;

use farmfe_core::{
  config::Config,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult, PluginResolveHookResult},
  serde_json,
};
use std::collections::HashMap;
use std::path::Path;
use utils::{normalize_path, path_join};

use farmfe_macro_plugin::farm_plugin;
#[derive(Debug)]
#[farm_plugin]
pub struct FarmPluginVirtualModule {
  virtual_options: HashMap<String, String>,
  resolved_ids: HashMap<String, String>,
}
const PREFIX: &str = "\0virtual:";
impl FarmPluginVirtualModule {
  fn new(_: &Config, options: String) -> Self {
    let virtual_options = serde_json::from_str::<HashMap<String, String>>(&options).unwrap();
    let mut resolved_ids: HashMap<String, String> = HashMap::new();
    for (module_id, module_content) in &virtual_options {
      resolved_ids.insert(
        utils::resolve_path(module_id.to_string().clone()),
        module_content.clone(),
      );
    }
    Self {
      virtual_options,
      resolved_ids,
    }
  }
}
impl Plugin for FarmPluginVirtualModule {
  fn name(&self) -> &str {
    "FarmPluginVirtual"
  }
  fn resolve(
    &self,
    param: &farmfe_core::plugin::PluginResolveHookParam,
    context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginResolveHookResult>> {
    let root = &context.config.root;
    if self.virtual_options.get(&param.source).is_some() {
      let result: PluginResolveHookResult = PluginResolveHookResult {
        resolved_path: format!("{}{}", PREFIX, param.source),
        ..Default::default()
      };
      return Ok(Some(result));
    }
    if let Some(id) = &param.importer {
      let parts = [&root, id.relative_path()];
      let absolute_path = path_join(&parts);
      let origin_import = absolute_path.as_str();
      let mut importer_no_prefix = origin_import;
      if id.relative_path().starts_with(PREFIX) {
        importer_no_prefix = importer_no_prefix.strip_prefix(PREFIX).unwrap();
      }
      let resolved = Path::new(importer_no_prefix).with_file_name(&param.source);
      let resolved = normalize_path(resolved).to_string_lossy().to_string();
      if self.resolved_ids.get(resolved.as_str()).is_some() {
        return Ok(Some(PluginResolveHookResult {
          resolved_path: format!("{}{}", PREFIX, resolved),
          ..Default::default()
        }));
      }
    }
    Ok(None)
  }
  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    let resolved_path = param.resolved_path;
    if resolved_path.starts_with(PREFIX) {
      let id = resolved_path.strip_prefix(PREFIX).unwrap();
      if let Some(value) = self.virtual_options.get(id) {
        return Ok(Some(PluginLoadHookResult {
          content: value.clone(),
          module_type: ModuleType::Js,
          source_map: None,
        }));
      } else {
        if let Some(value) = self.resolved_ids.get(id) {
          return Ok(Some(PluginLoadHookResult {
            content: value.clone(),
            module_type: ModuleType::Js,
            source_map: None,
          }));
        }
      }
    }
    Ok(None)
  }
}
// test build