use super::scan_exports::{
  Import,
  scan_exports
};
use walkdir::WalkDir;

pub fn scan_dir_exports(dir: &str) -> Vec<Import> {
  let walker = WalkDir::new(dir).into_iter();
  let file_exts = vec![".js", ".ts", ".jsx", ".tsx"];
  let filtered_entries = walker.filter_map(Result::ok).filter(|e| {
    e.file_type().is_file()
      && e.path().extension().is_some()
      && file_exts.contains(&e.path().extension().unwrap().to_str().unwrap())
  });

  let mut exports = Vec::new();
  for entry in filtered_entries {
    let file_path = entry.path();
    let content = std::fs::read_to_string(file_path).unwrap();
    let exports_names = scan_exports(file_path.to_str().unwrap(), &content);
    exports.extend(exports_names);
  }
  exports
}
