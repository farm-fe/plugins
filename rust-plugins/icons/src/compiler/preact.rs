pub use svgr_rs::{transform as react_complier, Config, JSXRuntime};

use super::CompilerParams;

pub fn preact_complier(param: CompilerParams) -> String {
  let CompilerParams { svg, .. } = param;
  let code = react_complier(
    svg,
    Config {
      jsx_runtime: Some(JSXRuntime::ClassicPreact),
      ..Default::default()
    },
    Default::default(),
  )
  .unwrap();
  code
}
