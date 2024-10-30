#![deny(clippy::all)]

mod load_shader;

use lazy_static::lazy_static;

use load_shader::*;
use farmfe_core::{
  config::{Config, SourcemapConfig},
  plugin::Plugin,
  serde_json,
};
use farmfe_macro_plugin::farm_plugin;
use serde::Deserialize;

lazy_static! {
  static ref DEFAULT_EXTENSION: &'static str = "glsl";
  static ref DEFAULT_SHADERS: &'static [&'static str] = &[
    "**/*.glsl",
    "**/*.wgsl",
    "**/*.vert",
    "**/*.frag",
    "**/*.vs",
    "**/*.fs"
  ];
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FarmfePluginGlslOptions {
  include: Option<Vec<String>>,
  exclude: Option<Vec<String>>,
  warn_duplicated_imports: Option<bool>,
  default_extension: Option<String>,
  compress: Option<bool>,
  watch: Option<bool>,
  root: Option<String>,
}
#[farm_plugin]

pub struct FarmfePluginGlsl {
  include: Vec<String>,
  exclude: Vec<String>,
  warn_duplicated_imports: bool,
  default_extension: String,
  compress: bool,
  watch: bool,
  root: String,
  sourcemap: Box<SourcemapConfig>,
}

impl FarmfePluginGlsl {
  fn new(config: &Config, options: String) -> Self {
    let glsl_options: FarmfePluginGlslOptions = serde_json::from_str(&options).unwrap();

    let include = glsl_options
      .include
      .unwrap_or_else(|| DEFAULT_SHADERS.iter().map(|&s| s.to_string()).collect());
    let exclude = glsl_options.exclude.unwrap_or_else(Vec::new);
    let warn_duplicated_imports = glsl_options.warn_duplicated_imports.unwrap_or(true);
    let default_extension = glsl_options
      .default_extension
      .unwrap_or_else(|| DEFAULT_EXTENSION.to_string());
    let compress = glsl_options.compress.unwrap_or(false);
    let watch = glsl_options.watch.unwrap_or(false);
    let root = glsl_options.root.unwrap_or_else(|| String::from("/"));
    let sourcemap = config.sourcemap.clone();

    Self {
      include,
      exclude,
      warn_duplicated_imports,
      default_extension,
      compress,
      watch,
      root,
      sourcemap,
    }
  }
}

impl Plugin for FarmfePluginGlsl {
  fn name(&self) -> &str {
    "FarmfePluginGlsl"
  }
}
