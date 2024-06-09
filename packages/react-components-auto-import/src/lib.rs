#![deny(clippy::all)]

use farmfe_core::{config::Config, plugin::Plugin};

use farmfe_macro_plugin::farm_plugin;

#[farm_plugin]
pub struct ReactComponentsAutoImport {}

impl ReactComponentsAutoImport {
  fn new(config: &Config, options: String) -> Self {
    Self {}
  }
}

impl Plugin for ReactComponentsAutoImport {
  fn name(&self) -> &str {
    "ReactComponentsAutoImport"
  }
}
