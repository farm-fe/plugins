use std::path::PathBuf;
use tailwind_ast::parse_tailwind;
use tailwind_css::{TailwindBuilder, TailwindInstruction};
use tailwindcss_oxide::{ChangedContent, Scanner};

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

pub fn parse_oxide_string(file_path: &str) -> String {
  let mut tailwind = TailwindBuilder::default();
  let mut scanner = Scanner::new(None, None);
  let res = scanner.scan_content(
    [ChangedContent {
      file: Some(PathBuf::from(file_path)),
      content: None,
    }]
    .to_vec(),
  );
  let styles: String = filter_tailwind_atom_css(res).join(" ");
  tailwind.trace(&styles, false).unwrap();
  let bundle = tailwind.bundle().unwrap();
  bundle
}
