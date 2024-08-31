#![deny(clippy::all)]
use farmfe_core::{config::Config, plugin::Plugin};
use farmfe_macro_plugin::farm_plugin;
use std::path::PathBuf;
use tailwind_ast::parse_tailwind;
use tailwind_css::{TailwindBuilder, TailwindInstruction};
use tailwindcss_oxide::{ChangedContent, Scanner};
#[farm_plugin]
pub struct FarmfePluginTailwindcss {}
impl FarmfePluginTailwindcss {
  fn new(config: &Config, options: String) -> Self {
    parse_oxide_string();
    Self {}
  }
}

impl Plugin for FarmfePluginTailwindcss {
  fn name(&self) -> &str {
    "FarmfePluginTailwindcss"
  }
}

fn parse_oxide_string() -> String {
  fn filter_tailwind_atom_css(css: Vec<String>) -> Vec<String> {
    let need_filter = vec![
      "content".to_string(),
      "word-break".to_string(),
      "text-overflow".to_string(),
    ];
    css
      .iter()
      .filter(|&c| {
        let styles = parse_tailwind(c).unwrap()[0].clone();
        if need_filter.contains(c) {
          return false;
        }
        TailwindInstruction::from(styles).get_instance().is_ok()
      })
      .cloned()
      .collect()
  }

  let mut tailwind = TailwindBuilder::default();
  let mut scanner = Scanner::new(None, None);
  let res = scanner.scan_content(
    [ChangedContent {
      file: Some(PathBuf::from(
        "/Users/cherry7/Documents/open/farm-fe/plugins/packages/tailwindcss/playground/index.html",
      )),
      content: None,
    }]
    .to_vec(),
  );
  let styles: String = filter_tailwind_atom_css(res).join(" ");
  tailwind.trace(&styles, false).unwrap();
  let bundle = tailwind.bundle().unwrap();
  bundle
}
