use std::path::PathBuf;
use tailwind_ast::parse_tailwind;
use tailwind_css::{TailwindBuilder, TailwindInstruction};
use tailwindcss_oxide::{scanner::detect_sources::DetectSources, GlobEntry, Scanner};
pub fn filter_tailwind_atom_css(css: Vec<String>) -> Vec<String> {
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

pub fn parse_tailwind_css(base: &str, contents: Vec<String>) -> String {
  let mut tailwind = TailwindBuilder::default();
  let sources = contents
    .iter()
    .map(|c| {
      return GlobEntry {
        base: base.to_string(),
        pattern: c.clone(),
      };
    })
    .collect();
  let mut scanner = Scanner::new(Some(DetectSources::new(PathBuf::from(base))), Some(sources));
  let res = scanner.scan();
  let styles: String = filter_tailwind_atom_css(res).join(" ");
  tailwind.trace(&styles, false).unwrap();
  let bundle = tailwind.bundle().unwrap();
  println!("bundle: {:?}", bundle);
  bundle
}
