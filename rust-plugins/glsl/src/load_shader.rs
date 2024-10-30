use lazy_static::lazy_static;

use crate::FarmfePluginGlslOptions;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
  static ref recursive_chunk: Mutex<String> = Mutex::new(String::new());
  static ref all_chunks: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
  static ref dependent_chunks: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
  static ref duplicated_chunks: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
  static ref INCLUDE_REGEX: Regex = Regex::new(r"#include(\s+([^\s<>]+));?").unwrap();
}

fn reset_saved_chunks() -> String {
  let mut chunks = recursive_chunk.lock().unwrap();
  let copy_chunks = chunks.clone();
  all_chunks.lock().unwrap().clear();
  dependent_chunks.lock().unwrap().clear();
  duplicated_chunks.lock().unwrap().clear();
  *chunks = String::new();
  copy_chunks
}

pub fn load_shader(source: &str, shader: &str, options: &FarmfePluginGlslOptions) {
  let FarmfePluginGlslOptions {
    compress,
    root,
    default_extension,
    warn_duplicated_imports,
    ..
  } = options;

  reset_saved_chunks();
}
