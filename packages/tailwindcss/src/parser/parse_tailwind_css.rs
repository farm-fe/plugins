use farmfe_toolkit::fs::read_file_utf8;
use std::path::PathBuf;
use tailwind_ast::parse_tailwind;
use tailwind_css::{TailwindBuilder, TailwindInstruction};
use tailwindcss_oxide::{ChangedContent, Scanner};

pub fn filter_tailwind_atom_css(css: Vec<String>) -> Vec<String> {
  css
    .iter()
    .filter(|&c| {
      let styles = parse_tailwind(c).unwrap()[0].clone();
      match TailwindInstruction::from(styles).get_instance() {
        Ok(rule) => {
          if rule.id().contains("[]") {
            // ignore atomic rules that do not conform to the specification
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

pub fn collect_tailwind_css(tw_builder: &mut TailwindBuilder, tw_scanner: &mut Scanner) {
  let rules = tw_scanner.scan();
  let styles = filter_tailwind_atom_css(rules);
  let styles_str = styles.join(" ");
  if styles_str.is_empty() {
    println!("Unsupported atomic rules: {:#?}", styles);
    return;
  }
  tw_builder.trace(&styles_str, false).unwrap();
}

pub fn collect_tailwind_css_with_changed(
  tw_builder: &mut TailwindBuilder,
  tw_scanner: &mut Scanner,
  changed_files: Vec<String>,
)-> bool {
  let changed_content = changed_files
    .iter()
    .map(|c| {
      return ChangedContent {
        file: Some(PathBuf::from(c)),
        content: Some(read_file_utf8(c).unwrap()),
      };
    })
    .collect();
  let rules = tw_scanner.scan_content(changed_content);
  let styles = filter_tailwind_atom_css(rules);
  let styles_str = styles.join(" ");
  if styles_str.is_empty() {
    println!("Unsupported atomic rules: {:#?}", styles);
    return false;
  }
  tw_builder.trace(&styles_str, false).unwrap();
  true
}
