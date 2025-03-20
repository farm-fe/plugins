#![deny(clippy::all)]

use farmfe_core::{config::Config, plugin::Plugin};

use farmfe_macro_plugin::farm_plugin;

#[farm_plugin]
pub struct FarmPluginVue {}

impl FarmPluginVue {
  fn new(config: &Config, options: String) -> Self {
    Self {}
  }
}

impl Plugin for FarmPluginVue {
  fn name(&self) -> &str {
    "FarmPluginVue"
  }
}
