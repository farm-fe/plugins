#![deny(clippy::all)]
mod parse;

use farmfe_core::{config::Config, plugin::Plugin};
use farmfe_macro_plugin::farm_plugin;
use parse::parse_oxide_string;
#[farm_plugin]
pub struct FarmfePluginTailwindcss {}
impl FarmfePluginTailwindcss {
  fn new(config: &Config, options: String) -> Self {
    parse_oxide_string();
    Self {}
  }
}

impl Plugin for FarmfePluginTailwindcss {
  fn name(&self) -> &str {
    "FarmfePluginTailwindcss"
  }
}
