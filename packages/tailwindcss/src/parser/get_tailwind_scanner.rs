use std::path::PathBuf;

use tailwindcss_oxide::{scanner::detect_sources::DetectSources, GlobEntry, Scanner};

pub fn get_tailwind_scanner(base: &str, contents: Vec<String>) -> Scanner {
  let sources = contents
    .iter()
    .map(|c| {
      return GlobEntry {
        base: base.to_string(),
        pattern: c.clone(),
      };
    })
    .collect();
  Scanner::new(Some(DetectSources::new(PathBuf::from(base))), Some(sources))
}
