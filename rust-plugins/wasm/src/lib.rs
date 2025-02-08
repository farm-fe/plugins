pub mod utils;

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
use utils::{generate_glue_code, GlueNames};

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

    Ok(None)
  }

  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    let resolved_path = param.resolved_path;
    if resolved_path == WASM_HELPER_ID_FARM {
      return Ok(Some(PluginLoadHookResult {
        content: include_str!("wasm_runtime.js").to_string(),
        module_type: ModuleType::Ts,
        source_map: None,
      }));
    }

    if resolved_path.ends_with(".wasm") {
      let init = param.query.iter().any(|(k, _)| k == "init");
      let content = fs::read(resolved_path).unwrap();
      let file_name_ext = Path::new(resolved_path)
        .file_name()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap();
      let (file_name, ext) = file_name_ext.split_once('.').unwrap();
      let assets_filename_config = context.config.output.assets_filename.clone();

      let output_file_name = transform_output_filename(
        assets_filename_config,
        file_name,
        param.module_id.as_bytes(),
        ext,
      );
      let params = EmitFileParams {
        name: output_file_name.clone(),
        content,
        resource_type: ResourceType::Asset("wasm".to_string()),
        resolved_path: param.module_id.to_string(),
      };
      context.emit_file(params);

      // let wasm_url = if !context.config.output.public_path.is_empty() {
      //   let normalized_public_path = context.config.output.public_path.trim_end_matches('/');
      //   format!("{}/{}", normalized_public_path, resolved_path)
      // } else {
      //   format!("/{}", resolved_path)
      // };

      let wasm_url = resolved_path;
      let content = if init {
        format!(
          r#"import initWasm from "{WASM_HELPER_ID_FARM}";
          import wasmUrl from "{wasm_url}?url";
          export default opts => initWasm(opts, wasmUrl)"#,
        )
      } else {
        format!(
          r#"URL = globalThis.URL;
          import initWasm from "{WASM_HELPER_ID_FARM}";
          import wasmUrl from "{wasm_url}?url";
          {}
          "#,
          generate_glue_code(
            param.resolved_path,
            &GlueNames {
              init_wasm: "initWasm".to_string(),
              wasm_url: "wasmUrl".to_string()
            }
          )
          .unwrap()
        )
      };
      return Ok(Some(PluginLoadHookResult {
        content,
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
