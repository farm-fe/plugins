use fervid::{compile, CompileOptions, CompileResult};

pub fn vue_compile(code: &str, file_name: &str, id: &str) -> String {
  let CompileResult { code, .. } = compile(
    &code,
    CompileOptions {
      filename: std::borrow::Cow::Borrowed(&file_name),
      id: std::borrow::Cow::Borrowed(&id),
      is_prod: Some(true),
      ssr: Some(false),
      source_map: None,
      gen_default_as: None,
    },
  )
  .unwrap();
  code
}
