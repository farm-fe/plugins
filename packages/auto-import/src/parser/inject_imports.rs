use crate::parser::parse::{parse_esm_imports, ESMImport};

use super::scan_exports::Import;
use super::stringify_imports::stringify_imports;
pub fn inject_imports(content: &str, imports: Vec<Import>, priority: Option<usize>) -> String {
  let esm_imports = parse_esm_imports(None, Some(content));
  let imports = imports
    .into_iter()
    .filter(|import| {
      !esm_imports.iter().any(|esm_import| {
        let ESMImport {
          named_imports,
          default_import,
          namespaced_import,
          type_named_imports,
          ..
        } = esm_import;
        let Import {
          name: import_name,
          priority: import_priority,
          ..
        } = import;
        let c_priority = priority.unwrap_or(1) - import_priority;
        if let Some(named_import) = named_imports {
          let import_keys: Vec<String> = named_import.keys().cloned().collect();
          if import_keys.contains(&import_name) {
            if c_priority == 0 {
              println!(
                "{}",
                format!(
                  "Duplicated in imported, has been ignored and {} is used",
                  import_name
                )
              );
              return false;
            } else {
              return true;
            }
          }
        }
        if let Some(type_named_import) = type_named_imports {
          let import_keys: Vec<String> = type_named_import.keys().cloned().collect();
          if import_keys.contains(&import_name) {
            return true;
          }
        }
        if let Some(default_import) = default_import {
          if default_import == import_name {
            if c_priority == 0 {
              println!(
                "{}",
                format!(
                  "Duplicated in imported, has been ignored and {} is used",
                  import_name
                )
              );
              return false;
            } else {
              return true;
            }
          }
        }
        if let Some(namespaced_import) = namespaced_import {
          if namespaced_import == import_name {
            if c_priority == 0 {
              println!(
                "{}",
                format!(
                  "Duplicated in imported, has been ignored and {} is used",
                  import_name
                )
              );
              return false;
            } else {
              return true;
            }
          }
        }
        false
      })
    })
    .collect::<Vec<Import>>();

  let mut content_str = stringify_imports(imports);
  content_str.push_str(content);
  content_str
}
