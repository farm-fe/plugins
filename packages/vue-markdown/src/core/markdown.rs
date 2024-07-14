use crate::options::Options;
use markdown_it::MarkdownIt;
pub fn add() -> bool {
  true
}

pub fn create_markdown(options: Options) -> string {
  let is_vue2 = if let Some(vue_version) = options.vue_version {
    vue_version.starts_with("2.")
  } else {
    false
  };

  let md = MarkdownIt::new();
  ""
}
