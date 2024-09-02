use std::path::PathBuf;
use tailwind_ast::parse_tailwind;
use tailwind_css::{TailwindBuilder, TailwindInstruction};
use tailwindcss_oxide::{
  scanner::detect_sources::DetectSources, ChangedContent, GlobEntry, Scanner,
};

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

pub fn parse_tailwind_css(
  tw_builder: &mut TailwindBuilder,
  base: &str,
  contents: Vec<String>,
) -> (String, Scanner) {
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
  tw_builder.trace(&styles, false).unwrap();
  let bundle = tw_builder.bundle().unwrap();
  (bundle, scanner)
}

pub fn parse_tailwind_css_with_changed(
  tw_builder: &mut TailwindBuilder,
  scanner: &mut Scanner,
  changed_files: Vec<String>,
) -> String {
  let changed_content = changed_files
    .iter()
    .map(|c| {
      return ChangedContent {
        file: Some(PathBuf::from(c)),
        content: None,
      };
    })
    .collect();
  let res = scanner.scan_content(changed_content);
  let styles: String = filter_tailwind_atom_css(res).join(" ");
  tw_builder.trace(&styles, false).unwrap();
  let bundle = tw_builder.bundle().unwrap();
  bundle
}
