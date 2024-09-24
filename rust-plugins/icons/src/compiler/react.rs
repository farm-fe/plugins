pub use svgr_rs::{transform as _react_complier, Config, JSXRuntime};

use super::CompilerParams;

pub fn react_complier(param: CompilerParams) -> String {
  let CompilerParams { svg, .. } = param;
  let code = _react_complier(
    svg,
    Config {
      jsx_runtime: Some(JSXRuntime::Classic),
      ..Default::default()
    },
    Default::default(),
  )
  .unwrap();
  code
}
