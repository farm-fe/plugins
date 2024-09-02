#![deny(clippy::all)]
mod config;
mod parser;

use std::sync::{Arc, Mutex};

use config::TailwindCssConfig;
use farmfe_core::{
  config::Config,
  plugin::Plugin,
  serde_json::{self},
};
use farmfe_macro_plugin::farm_plugin;
use parser::{
  get_tailwind_builder::get_tailwind_builder,
  parse_tailwind_css::{parse_tailwind_css, parse_tailwind_css_with_changed},
};
use tailwind_css::TailwindBuilder;
use tailwindcss_oxide::Scanner;

#[farm_plugin]
pub struct FarmfePluginTailwindcss {
  tw_config: TailwindCssConfig,
  tw_builder: Arc<Mutex<TailwindBuilder>>,
  tw_scanner: Arc<Mutex<Scanner>>,
  tw_bundle: Arc<Mutex<String>>,
}
impl FarmfePluginTailwindcss {
  fn new(config: &Config, options: String) -> Self {
    let base = config.root.clone();
    let tw_config: TailwindCssConfig = serde_json::from_str(&options).unwrap();
    let contents = tw_config.content.clone();
    let mut tw_builder = get_tailwind_builder(&tw_config);
    let (tw_bundle, tw_scanner) = parse_tailwind_css(&mut tw_builder, &base, contents.unwrap());

    Self {
      tw_config,
      tw_builder: Arc::new(Mutex::new(tw_builder)),
      tw_scanner: Arc::new(Mutex::new(tw_scanner)),
      tw_bundle: Arc::new(Mutex::new(tw_bundle)),
    }
  }
}

impl Plugin for FarmfePluginTailwindcss {
  fn name(&self) -> &str {
    "FarmfePluginTailwindcss"
  }
}
