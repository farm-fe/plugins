#![deny(clippy::all)]
mod parser;
mod presets;

use farmfe_core::{config::{config_regex::ConfigRegex, Config}, plugin::Plugin};

use farmfe_macro_plugin::farm_plugin;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum ImportMode {
  Relative,
  Absolute,
}

pub struct Options {
  pub imports: Option<Vec<String>>,
  pub dirs: Option<Vec<ConfigRegex>>,
  pub filename: Option<String>,
  pub dts: Option<bool>,
  pub local: Option<bool>,
  pub import_mode: Option<ImportMode>,
  pub include: Option<Vec<ConfigRegex>>,
  pub exclude: Option<Vec<ConfigRegex>>,
}

#[farm_plugin]
pub struct FarmfePluginAutoImport {}

impl FarmfePluginAutoImport {
  fn new(config: &Config, options: String) -> Self {
    Self {}
  }
}

impl Plugin for FarmfePluginAutoImport {
  fn name(&self) -> &str {
    "FarmfePluginAutoImport"
  }
}
