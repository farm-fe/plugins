#![deny(clippy::all)]
mod addons;
mod finish_imports;
mod parser;
mod presets;

use std::sync::{Arc, Mutex};

use farmfe_core::{
  config::{config_regex::ConfigRegex, Config},
  module::ModuleType,
  plugin::Plugin,
  serde_json,
};

use addons::vue_template::vue_template_addon;
use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::common::PathFilter;
use finish_imports::FinishImportsParams;
use parser::scan_exports::Import;
use presets::PresetItem;
use serde::{Deserialize, Serialize};

// Use shared utilities
use farm_plugin_shared::{Dts, init_plugin};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum ImportMode {
  Relative,
  Absolute,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Options {
  pub dirs: Option<Vec<ConfigRegex>>,
  pub dts: Option<Dts>,
  pub ignore: Option<Vec<ConfigRegex>>,
  pub presets: Option<Vec<PresetItem>>,
  pub import_mode: Option<ImportMode>,
  pub include: Option<Vec<ConfigRegex>>,
  pub exclude: Option<Vec<ConfigRegex>>,
}

#[farm_plugin]
pub struct FarmfePluginAutoImport {
  options: Arc<Options>, // Use Arc to avoid cloning
  collect_imports: Arc<Mutex<Vec<Import>>>,
}

impl FarmfePluginAutoImport {
  fn new(config: &Config, options: String) -> Self {
    let options: Options = init_plugin(&options).unwrap_or_else(|e| {
      panic!("Failed to initialize auto-import plugin: {}", e);
    });
    let collect_imports: Arc<Mutex<Vec<Import>>> = Arc::new(Mutex::new(Vec::new()));
    let dirs = options.dirs.clone().unwrap_or_default();
    let root_path = config.root.clone();
    let presets = options.presets.clone().unwrap_or_default();
    let ignore = options.ignore.clone().unwrap_or_default();
    finish_imports::finish_imports(FinishImportsParams {
      root_path,
      presets,
      dirs,
      ignore,
      dts: options.dts.clone().unwrap_or_default(),
      context_imports: &collect_imports,
    });
    Self {
      options: Arc::new(options),
      collect_imports,
    }
  }
}

impl Plugin for FarmfePluginAutoImport {
  fn name(&self) -> &str {
    "FarmfePluginAutoImport"
  }
  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    _context: &Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    if ![
      ModuleType::Jsx,
      ModuleType::Tsx,
      ModuleType::Js,
      ModuleType::Ts,
    ]
    .contains(&param.module_type)
    {
      return Ok(None);
    }
    let default_include = vec![];
    let default_exclude = vec![ConfigRegex::new("node_modules")];
    let include = self.options.include.as_ref().unwrap_or(&default_include);
    let exclude = self.options
      .exclude
      .as_ref()
      .unwrap_or(&default_exclude);
    let filter = PathFilter::new(include, exclude);
    if !filter.execute(&param.module_id) {
      return Ok(None);
    } else {
      let imports = self.collect_imports.lock().unwrap();
      let mut content = param.content.clone();
      if param.resolved_path.ends_with(".vue") {
        vue_template_addon(&mut content, &imports);
      }
      let content =
        parser::inject_imports::inject_imports(&content, imports.to_vec(), None);
      // let (cm, src) = create_swc_source_map(Source {
      //   path: PathBuf::from(param.resolved_path),
      //   content: Arc::new(content.clone()),
      // });
      // let map = {
      //   let map = build_source_map(cm, &src_map);
      //   let mut buf = vec![];
      //   map.to_writer(&mut buf).expect("failed to write sourcemap");
      //   Some(String::from_utf8(buf).unwrap())
      // };
      Ok(Some(farmfe_core::plugin::PluginTransformHookResult {
        content,
        source_map: None,
        module_type: Some(param.module_type.clone()),
        ignore_previous_source_map: false,
      }))
    }
  }

  fn update_finished(
    &self,
    context: &Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<()>> {
    let dirs = self.options.dirs.clone().unwrap_or(vec![]);
    let root_path = context.config.root.clone();
    let presets = self.options.presets.clone().unwrap_or(vec![]);
    let ignore = self.options.ignore.clone().unwrap_or(vec![]);
    finish_imports::finish_imports(FinishImportsParams {
      root_path,
      presets,
      dirs,
      ignore,
      dts: self.options.dts.clone().unwrap_or_default(),
      context_imports: &self.collect_imports,
    });
    Ok(None)
  }
}
