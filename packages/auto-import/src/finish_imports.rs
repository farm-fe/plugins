use std::sync::{Arc, Mutex};

use farmfe_core::config::config_regex::ConfigRegex;

use crate::parser::generate_dts::{generate_dts, GenerateDtsOption};
use crate::parser::scan_dirs_exports::scan_dirs_exports;
use crate::parser::scan_exports::Import;
use crate::presets::resolve_presets;

pub struct FinishImportsParams<'a> {
  pub root_path: String,
  pub presets: Vec<String>,
  pub dirs: Vec<ConfigRegex>,
  pub filename: String,
  pub dts: bool,
  pub context_imports: &'a Arc<Mutex<Vec<Import>>>,
}

fn maybe_has_new_or_removed_imports(
  old_imports: &Vec<Import>,
  local_imports: &Vec<Import>,
  resolvers_imports: &Vec<Import>,
) -> bool {
  let old_len = old_imports.len();
  let new_len = local_imports.len() + resolvers_imports.len();
  old_len != new_len
    || local_imports
      .iter()
      .any(|import| !old_imports.contains(import))
    || resolvers_imports
      .iter()
      .any(|import| !old_imports.contains(import))
}

pub fn finish_imports(params: FinishImportsParams) {
  let FinishImportsParams {
    root_path,
    presets,
    dirs,
    filename,
    dts,
    context_imports,
  } = params;

  let mut local_imports = scan_dirs_exports(&root_path, &dirs.clone());
  let presets_imports = resolve_presets(&presets);
  let mut context_imports_guard = match context_imports.lock() {
    Ok(guard) => guard,
    Err(poisoned) => poisoned.into_inner(),
  };
  let has_new_or_removed_imports =
    maybe_has_new_or_removed_imports(&context_imports_guard, &local_imports, &presets_imports);
  if has_new_or_removed_imports && dts {
    let generate_dts_option = GenerateDtsOption {
      filename,
      root_path,
      imports: &local_imports.iter().collect::<Vec<_>>(),
      presets_imports: &presets_imports.iter().collect::<Vec<_>>(),
    };
    generate_dts(generate_dts_option);
  }
  if has_new_or_removed_imports {
    local_imports.extend(presets_imports);
    *context_imports_guard = local_imports;
  }
}
