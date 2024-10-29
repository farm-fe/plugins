#![deny(clippy::all)]

use farmfe_core::{
  cache_item,
  config::Config,
  context::{CompilationContext, EmitFileParams},
  deserialize,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult, PluginResolveHookResult},
  resource::{Resource, ResourceOrigin, ResourceType},
  serialize,
};
use std::{fs, path::Path, sync::Arc};

use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::fs::transform_output_filename;
const WASM_HELPER_ID_FARM: &str = "farm/wasm-helper.js";

#[cache_item]
struct CachedStaticAssets {
  list: Vec<Resource>,
}

#[farm_plugin]
pub struct FarmfePluginWasm {}

impl FarmfePluginWasm {
  fn new(_config: &Config, _options: String) -> Self {
    Self {}
  }
}

impl Plugin for FarmfePluginWasm {
  fn name(&self) -> &str {
    "FarmfePluginWasm"
  }
  fn resolve(
    &self,
    param: &farmfe_core::plugin::PluginResolveHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginResolveHookResult>> {
    let id = &param.source;
    if id == WASM_HELPER_ID_FARM {
      return Ok(Some(PluginResolveHookResult {
        resolved_path: id.to_string(),
        ..Default::default()
      }));
    }

    // if id.ends_with(".wasm?init") {
    //   return Ok(Some(PluginResolveHookResult {
    //     resolved_path: id.replace("?init", ""),
    //     query: vec![("init".to_string(), "".to_string())]
    //       .into_iter()
    //       .collect(),
    //     ..Default::default()
    //   }));
    // }

    // if id.ends_with(".wasm?url") {
    //   return Ok(Some(PluginResolveHookResult {
    //     resolved_path: id.to_string(),
    //     query: vec![("url".to_string(), "".to_string())]
    //       .into_iter()
    //       .collect(),
    //     ..Default::default()
    //   }));
    // }

    Ok(None)
  }

  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    if param.resolved_path == WASM_HELPER_ID_FARM {
      return Ok(Some(PluginLoadHookResult {
        content: include_str!("wasm_runtime.js").to_string(),
        module_type: ModuleType::Js,
        source_map: None,
      }));
    }

    if param.resolved_path.ends_with(".wasm") {
      let query = &param.query;
      let init = query.iter().any(|(k, _)| k == "init");
      let content = fs::read(&param.resolved_path).unwrap();
      let file_name_ext = Path::new(&param.resolved_path)
        .file_name()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap();
      let (file_name, ext) = file_name_ext.split_once(".").unwrap();
      let assets_filename_config = context.config.output.assets_filename.clone();
      let wasm_url = if !context.config.output.public_path.is_empty() {
        let normalized_public_path = context.config.output.public_path.trim_end_matches("/");
        format!("{}/{}", normalized_public_path, file_name)
      } else {
        format!("/{}", file_name)
      };
      let file_name =
        transform_output_filename(assets_filename_config, &file_name, param.module_id.as_bytes(), ext);
      let params = EmitFileParams {
        name: file_name,
        content,
        resource_type: ResourceType::Asset("wasm".to_string()),
        resolved_path: param.module_id.to_string(),
      };
      context.emit_file(params);
      let mut _code = String::new();
      if init {
        _code = format!(
          r#"import initWasm from "{WASM_HELPER_ID_FARM}"; 
          export default opts => initWasm(opts, "{wasm_url}")"#
        );
      } else {
        _code = format!(
          r#"import initWasm from "{WASM_HELPER_ID_FARM}"; 
        const instance = await initWasm(undefined, "{wasm_url}");
        Object.assign(exports, instance.exports);
        export default instance;
        "#
        );
      }
      return Ok(Some(PluginLoadHookResult {
        content: _code,
        module_type: ModuleType::Js,
        source_map: None,
      }));
    }
    Ok(None)
  }

  fn plugin_cache_loaded(
    &self,
    cache: &Vec<u8>,
    context: &Arc<CompilationContext>,
  ) -> farmfe_core::error::Result<Option<()>> {
    let cached_static_assets = deserialize!(cache, CachedStaticAssets);

    for asset in cached_static_assets.list {
      if let ResourceOrigin::Module(m) = asset.origin {
        context.emit_file(EmitFileParams {
          resolved_path: m.to_string(),
          name: asset.name,
          content: asset.bytes,
          resource_type: asset.resource_type,
        });
      }
    }

    Ok(Some(()))
  }
  fn write_plugin_cache(
    &self,
    context: &Arc<CompilationContext>,
  ) -> farmfe_core::error::Result<Option<Vec<u8>>> {
    let mut list = vec![];
    let resources_map = context.resources_map.lock();
    for (_, resource) in resources_map.iter() {
      if let ResourceOrigin::Module(m) = &resource.origin {
        if context.cache_manager.module_cache.has_cache(m) {
          list.push(resource.clone());
        }
      }
    }

    if !list.is_empty() {
      let cached_static_assets = CachedStaticAssets { list };
      Ok(Some(serialize!(&cached_static_assets)))
    } else {
      Ok(None)
    }
  }
}
