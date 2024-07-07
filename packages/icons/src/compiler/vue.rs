use super::CompilerParams;

pub fn vue_compiler(param: CompilerParams) -> String {
  let CompilerParams { svg:_svg, .. } = param;
  // compile_sync_naive(code, true).unwrap()
  String::new()
}
