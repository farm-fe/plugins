use farmfe_core::{
  config::config_regex::ConfigRegex,
  swc_ecma_ast::*,
  swc_ecma_parser::{Syntax, TsSyntax},
};
use farmfe_toolkit::{
  common::PathFilter,
  pluginutils::normalize_path::normalize_path,
  script::{parse_module, ParseScriptModuleResult},
  swc_ecma_visit::{Visit, VisitWith},
};
pub fn scan_exports(content: &str) -> Vec<String> {
  
  todo!()
}
