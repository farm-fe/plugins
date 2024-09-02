use farmfe_toolkit::fs::read_file_utf8;
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
      if need_filter.contains(c) {
        return false;
      }
      let styles = parse_tailwind(c).unwrap()[0].clone();
      match TailwindInstruction::from(styles).get_instance() {
        Ok(rule) => {
          if rule.id().contains("[]") { // ignore atomic rules that do not conform to the specification
            return false;
          }
          return true;
        }
        Err(_e) => {
          return false;
        }
      }
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
  let rules = scanner.scan();
  if rules.is_empty() {
    return (String::new(), scanner);
  }
  let styles = filter_tailwind_atom_css(rules);
  if styles.is_empty() {
    return (String::new(), scanner);
  }
  let styles_str = styles.join(" ");
  if styles_str.is_empty() {
    println!("Unsupported atomic rules: {:#?}", styles);
    return (String::new(), scanner);
  }
  tw_builder.trace(&styles_str, false).unwrap();
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
        content: Some(read_file_utf8(c).unwrap()),
      };
    })
    .collect();
  let rules = scanner.scan_content(changed_content);
  if rules.is_empty() {
    return String::new();
  }
  let styles = filter_tailwind_atom_css(rules);
  if styles.is_empty() {
    return String::new();
  }
  let styles_str = styles.join(" ");
  if styles_str.is_empty() {
    println!("Unsupported atomic rules: {:#?}", styles);
    return String::new();
  }
  tw_builder.trace(&styles_str, false).unwrap();
  let bundle = tw_builder.bundle().unwrap();
  bundle
}

fn clean_tailwind_css(css: &str) -> String {
  //
  todo!()
}
