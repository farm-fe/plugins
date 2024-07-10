#![deny(clippy::all)]
mod common;
mod compiler;
mod gen_svg;
mod options;
mod update_svg;
// mod svg_id;
use std::collections::HashMap;

use common::{
  get_icon_path_data, get_icon_path_data_by_custom_collections, get_icon_path_meta, is_icon_path,
  resolve_icons_path, GetIconCustomCollectionPathDataParams, GetIconPathDataParams,
};
use compiler::{get_compiler, get_module_type_by_path, CompilerParams, GetCompilerParams};
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
use serde_json::Value;
use update_svg::SvgModifier;

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
        "vue" => format!("{}.js", res),
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
      let root_path = self
        .options
        .collections_node_resolve_path
        .clone()
        .unwrap_or_default();

      let mut svg_raw = String::new();

      let meta = resolve_icons_path(source);
      let query_map = param.query.iter().cloned().collect::<HashMap<_, _>>();
      let custom_collections = self
        .options
        .custom_collections
        .clone()
        .unwrap_or(Value::Null);
      let custom_collection_path = custom_collections
        .get(&meta.collection)
        .and_then(|v| v.as_str());

      if custom_collection_path.is_some() {
        svg_raw = get_icon_path_data_by_custom_collections(GetIconCustomCollectionPathDataParams {
          custom_collection_path: custom_collection_path.unwrap().to_string(),
          icon: meta.icon.clone(),
          project_dir: root_path.clone(),
        });

        if !svg_raw.is_empty() {
          svg_raw = SvgModifier::new(SvgModifier {
            fill: query_map.get("fill").and_then(|v| v.parse().ok()),
            stroke: query_map.get("stroke").and_then(|v| v.parse().ok()),
            stroke_width: query_map.get("stroke-width").and_then(|v| v.parse().ok()),
            width: query_map.get("width").and_then(|v| v.parse().ok()),
            height: query_map.get("height").and_then(|v| v.parse().ok()),
            class: self.options.default_class.clone(),
            style: self.options.default_style.clone(),
          })
          .apply_to_svg(&svg_raw);
        }
      } else {
        let data = get_icon_path_data(GetIconPathDataParams {
          path: source.to_string(),
          project_dir: root_path.clone(),
          auto_install: self.options.auto_install.unwrap_or_default(),
        });

        if data.is_null() {
          return Ok(Some(PluginLoadHookResult {
            content: String::new(),
            module_type: ModuleType::Js,
            source_map: None,
          }));
        }

        let svg_path_str: Option<String> =
          data.get("body").and_then(|v| v.as_str().map(String::from));
        let svg_data_height: Option<String> = data
          .get("height")
          .and_then(|v| v.as_number().map(|v| v.to_string()));
        let svg_data_width: Option<String> = data
          .get("width")
          .and_then(|v| v.as_number().map(|v| v.to_string()));

        let svg_el_builder = gen_svg::GenSvgElement::new(GenSvgElement {
          fill: query_map.get("fill").and_then(|v| v.parse().ok()),
          stroke: query_map.get("stroke").and_then(|v| v.parse().ok()),
          stroke_width: query_map.get("stroke-width").and_then(|v| v.parse().ok()),
          width: query_map
            .get("width")
            .and_then(|v| v.parse().ok())
            .or(svg_data_width),
          height: query_map
            .get("height")
            .and_then(|v| v.parse().ok())
            .or(svg_data_height),
          class: self.options.default_class.clone(),
          style: self.options.default_style.clone(),
          scale: self.options.scale,
          path: svg_path_str,
        });
        svg_raw = svg_el_builder.apply_to_svg();
      }
      if query_map.contains_key("raw") {
        return Ok(Some(PluginLoadHookResult {
          content: svg_raw,
          module_type: ModuleType::Asset,
          source_map: None,
        }));
      }
      let code = get_compiler(GetCompilerParams {
        jsx: self.options.jsx.clone().unwrap_or_default(),
        compiler: self.options.compiler.clone().unwrap_or_default(),
      })(CompilerParams {
        svg: svg_raw,
        root_path,
        svg_name: meta.icon,
      });
      let module_type = get_module_type_by_path(GetCompilerParams {
        jsx: self.options.jsx.clone().unwrap_or_default(),
        compiler: self.options.compiler.clone().unwrap_or_default(),
      });
      Ok(Some(PluginLoadHookResult {
        content: code,
        module_type,
        source_map: None,
      }))
    } else {
      Ok(None)
    }
  }
}
