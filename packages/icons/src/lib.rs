#![deny(clippy::all)]

use farmfe_core::{config::Config, plugin::Plugin};

use farmfe_macro_plugin::farm_plugin;

#[farm_plugin]
pub struct FarmfePluginIcons {}

impl FarmfePluginIcons {
  fn new(config: &Config, options: String) -> Self {
    Self {}
  }
}

impl Plugin for FarmfePluginIcons {
  fn name(&self) -> &str {
    "FarmfePluginIcons"
  }
}
