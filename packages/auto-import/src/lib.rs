#![deny(clippy::all)]
mod finish_imports;
mod parser;
mod presets;

use std::{
  path::PathBuf,
  sync::{Arc, Mutex},
};

use farmfe_core::{
  config::{config_regex::ConfigRegex, Config},
  module::ModuleType,
  plugin::Plugin,
  serde_json,
};

use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::common::{build_source_map, create_swc_source_map, PathFilter, Source};
use finish_imports::FinishImportsParams;
use parser::scan_exports::Import;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum ImportMode {
  Relative,
  Absolute,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Options {
  pub imports: Option<Vec<String>>,
  pub dirs: Option<Vec<ConfigRegex>>,
  pub filename: Option<String>,
  pub dts: Option<bool>,
  pub local: Option<bool>,
  pub presets: Option<Vec<String>>,
  pub import_mode: Option<ImportMode>,
  pub include: Option<Vec<ConfigRegex>>,
  pub exclude: Option<Vec<ConfigRegex>>,
}

#[farm_plugin]
pub struct FarmfePluginAutoImport {
  options: Options,
  collect_imports: Arc<Mutex<Vec<Import>>>,
}

impl FarmfePluginAutoImport {
  fn new(config: &Config, options: String) -> Self {
    let options: Options = serde_json::from_str(&options).unwrap();
    let collect_imports: Arc<Mutex<Vec<Import>>> = Arc::new(Mutex::new(vec![]));

    let presets = options.presets.clone().unwrap_or(vec![]);
    let filename = options
      .filename
      .clone()
      .unwrap_or("auto_import.d.ts".to_string());
    let dirs = options.dirs.clone().unwrap_or(vec![]);
    let root_path = config.root.clone();
    finish_imports::finish_imports(FinishImportsParams {
      root_path,
      presets,
      dirs,
      filename,
      dts: options.dts.unwrap_or(true),
      context_imports: &collect_imports,
    });
    Self {
      options,
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
    let options = self.options.clone();
    let include = options.include.unwrap_or(vec![]);
    let exclude = options.exclude.unwrap_or(vec![ConfigRegex::new("node_modules")]);
    let filter = PathFilter::new(&include, &exclude);
    if !filter.execute(&param.module_id) {
      return Ok(None);
    } else {
      let imports = self.collect_imports.lock().unwrap();
      let content =
        parser::inject_imports::inject_imports(&param.content, imports.clone().to_vec(), None);
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
    let filename = self
      .options
      .filename
      .clone()
      .unwrap_or("auto_import.d.ts".to_string());
    let dirs = self.options.dirs.clone().unwrap_or(vec![]);
    let root_path = context.config.root.clone();
    let presets = self.options.presets.clone().unwrap_or(vec![]);
    finish_imports::finish_imports(FinishImportsParams {
      root_path,
      presets,
      dirs,
      filename,
      dts: self.options.dts.unwrap_or(true),
      context_imports: &self.collect_imports,
    });
    Ok(None)
  }
}
