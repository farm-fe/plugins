#![deny(clippy::all)]
mod config;
mod parser;

use config::TailwindCssConfig;
use farmfe_core::{
  config::Config,
  plugin::Plugin,
  serde_json::{self},
};
use farmfe_macro_plugin::farm_plugin;

#[farm_plugin]
pub struct FarmfePluginTailwindcss {
  tw_config: TailwindCssConfig,
}
impl FarmfePluginTailwindcss {
  fn new(config: &Config, options: String) -> Self {
    let tw_config: TailwindCssConfig = serde_json::from_str(&options).unwrap(); 
    Self { tw_config }
  }
}

impl Plugin for FarmfePluginTailwindcss {
  fn name(&self) -> &str {
    "FarmfePluginTailwindcss"
  }
}
