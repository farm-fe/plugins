use std::{
  env,
  path::{Component, Path, PathBuf},
};

pub fn resolve_path(path: String) -> String {
  let mut absolute_path = PathBuf::new();
  absolute_path.push(env::current_dir().unwrap());
  absolute_path.push(path);
  absolute_path.into_os_string().into_string().unwrap()
}
pub fn path_join(parts: &[&str]) -> String {
  let mut path_buf = PathBuf::new();
  for part in parts {
    path_buf.push(part);
  }
  path_buf
    .canonicalize()
    .unwrap()
    .to_string_lossy()
    .to_string()
}
pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
  let ends_with_slash = path.as_ref().to_str().map_or(false, |s| s.ends_with('/'));
  let mut normalized = PathBuf::new();
  for component in path.as_ref().components() {
    match &component {
      Component::ParentDir => {
        if !normalized.pop() {
          normalized.push(component);
        }
      }
      _ => {
        normalized.push(component);
      }
    }
  }
  if ends_with_slash {
    normalized.push("");
  }
  normalized
}
