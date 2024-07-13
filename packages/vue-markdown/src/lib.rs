#![deny(clippy::all)]

mod utils;
mod options;
mod plugin_component;
use farmfe_core::{config::Config, plugin::Plugin};

use farmfe_macro_plugin::farm_plugin;

#[farm_plugin]
pub struct VueMarkdown {}

impl VueMarkdown {
  fn new(config: &Config, options: String) -> Self {
    Self {}
  }
}

impl Plugin for VueMarkdown {
  fn name(&self) -> &str {
    "VueMarkdown"
  }
}
