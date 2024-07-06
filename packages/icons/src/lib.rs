#![deny(clippy::all)]
mod common;
mod gen_svg;
mod options;
mod svg_id;

use common::{get_icon_path_data, get_icon_path_meta, is_icon_path, GetIconPathDataParams};
use farmfe_core::{
  config::Config,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult, PluginResolveHookResult},
  serde_json,
};
use farmfe_macro_plugin::farm_plugin;
use farmfe_utils::parse_query;
use gen_svg::GenSvgElement;
use options::Options;
use svgr_rs::transform as svgr_transform;

const PUBLIC_ICON_PREFIX: &str = "virtual:__FARM_ICON_ASSET__:";

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

    let jsx = options::guess_jsx(&config.root);

    Self {
      options: Options {
        collections_node_resolve_path,
        jsx: Some(jsx),
        ..options
      },
    }
  }
}

impl Plugin for FarmfePluginIcons {
  fn name(&self) -> &str {
    "FarmfePluginIcons"
  }
  fn resolve(
    &self,
    param: &farmfe_core::plugin::PluginResolveHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginResolveHookResult>> {
    if is_icon_path(&param.source) {
      let meta = get_icon_path_meta(&param.source);
      let res = meta.base_path.clone();
      let query = parse_query(&meta.query);
      let compiler = {
        if query.iter().any(|(k, _)| k == "raw") {
          "raw".to_string()
        } else {
          self
            .options
            .compiler
            .clone()
            .unwrap_or_else(|| "jsx".to_string())
        }
      };
      let resolved_path = match compiler.as_str() {
        "jsx" => format!("{}.jsx", res),
        "svelte" => format!("{}.svelte", res),
        "solid" => format!("{}.tsx", res),
        _ => res.clone(),
      };
      return Ok(Some(PluginResolveHookResult {
        resolved_path: format!("{PUBLIC_ICON_PREFIX}{}", resolved_path),
        external: false,
        side_effects: false,
        query: parse_query(&meta.query),
        ..Default::default()
      }));
    }
    Ok(None)
  }
  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    if let Some(source) = param.resolved_path.strip_prefix(PUBLIC_ICON_PREFIX) {
      let raw = get_icon_path_data(GetIconPathDataParams {
        path: source.to_string(),
        project_dir: self
          .options
          .collections_node_resolve_path
          .clone()
          .unwrap_or_default(),
        auto_install: self.options.auto_install.unwrap_or_default(),
      });
      let svg_el_builder = gen_svg::GenSvgElement::new(GenSvgElement {
        fill: None,
        stroke: None,
        stroke_width: None,
        width: None,
        height: None,
        path: Some(raw),
      });
      let el = svg_el_builder.apply_to_svg();
      let code = svgr_transform(el, Default::default(), Default::default()).unwrap();
      Ok(Some(PluginLoadHookResult {
        content: code,
        module_type: ModuleType::Jsx,
        source_map: None,
      }))
    } else {
      Ok(None)
    }
  }
}
