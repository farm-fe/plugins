use std::{fs::{metadata, read_dir, read_to_string}, path::Path};

use farmfe_toolkit::pluginutils::normalize_path::normalize_path;

use crate::parser::parse::{parse_esm_imports_exports, DeclarationType, ESMExport, ExportType};
const FILE_EXTENSION_LOOKUP: [&'static str; 8] = [
    ".mts",
    ".cts",
    ".ts",
    ".mjs",
    ".cjs",
    ".js",
    ".jsx",
    ".tsx"
];
pub struct Import {
  form: String,
  name: String,
  priority: usize,
  disabled: Option<bool>,
  dts_disabled: Option<bool>,
  declaration_type: Option<DeclarationType>,
  tp: Option<bool>,
  type_from: Option<String>,
  as_name: Option<String>,
}

fn to_pascal_case(s: &str) -> String {
  if s.contains('-') || s.contains('_') {
    s.split(|c| c == '-' || c == '_')
      .filter(|part| !part.is_empty())
      .map(|part| {
        let mut chars = part.chars();
        chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
      })
      .collect()
  } else {
    let chars = s.chars();
    chars.as_str().to_string()
  }
}

fn get_filename_by_path(file_path: &str) -> String {
  let path = Path::new(file_path);
  let filename = path
    .file_stem()
    .and_then(|filename_osstr| filename_osstr.to_str())
    .map(|filename_str| filename_str.to_owned())
    .unwrap();
  to_pascal_case(&filename)
}

pub fn scan_exports(file_path: &str, content: &str) -> Vec<Import> {
  let (_, exports) = parse_esm_imports_exports(None, Some(content));
  let filename = get_filename_by_path(file_path);
  let mut exports_names = Vec::new();
  for export in exports {
    let ESMExport {
      name,
      default_name,
      named_exports,
      export_type,
      type_named_exports,
      specifier,
      ..
    } = export;
    match export_type {
      ExportType::Default => {
        exports_names.push(Import {
          form: file_path.to_string(),
          name: default_name.unwrap(),
          priority: 0,
          disabled: None,
          dts_disabled: None,
          declaration_type: None,
          tp: None,
          type_from: None,
          as_name: None,
        });
      }
      ExportType::Type => {
        if let Some(type_named_export) = type_named_exports {
          for (_k, v) in type_named_export {
            exports_names.push(Import {
              form: file_path.to_string(),
              name: v,
              priority: 0,
              disabled: None,
              dts_disabled: None,
              declaration_type: None,
              tp: Some(true),
              type_from: Some(filename.clone()),
              as_name: None,
            });
          }
        }
      }
      ExportType::Declaration => {
        exports_names.push(Import {
          form: file_path.to_string(),
          name: name.unwrap(),
          priority: 0,
          disabled: None,
          dts_disabled: None,
          declaration_type: None,
          tp: None,
          type_from: None,
          as_name: None,
        });
      }
      ExportType::Namespace => exports_names.push(Import {
        form: file_path.to_string(),
        name: name.unwrap(),
        priority: 0,
        disabled: None,
        dts_disabled: None,
        declaration_type: None,
        tp: None,
        type_from: None,
        as_name: None,
      }),
      ExportType::Named => {
        if let Some(named_export) = named_exports {
          for (_k, v) in named_export {
            exports_names.push(Import {
              form: file_path.to_string(),
              name: v,
              priority: 0,
              disabled: None,
              dts_disabled: None,
              declaration_type: None,
              tp: None,
              type_from: None,
              as_name: None,
            });
          }
        }
      }
      ExportType::All=>{
        let specifier_path = Path::new(file_path).join(specifier.unwrap());
        let specifier_path = normalize_path(specifier_path.to_str().unwrap());

        let file_exts = FILE_EXTENSION_LOOKUP.to_vec();
        // check specifier_path is a directory
        if metadata(&specifier_path).unwrap().is_dir() {
          // check if specifier_path has index.tsx ...
          for ext in &file_exts {
            let index_path = format!("{}/index{}",specifier_path, ext);
            if metadata(&index_path).is_ok() {
              let index_content = read_to_string(&index_path).unwrap();
              let index_exports = scan_exports(&index_path, &index_content);
              exports_names.extend(index_exports);
              break;
            }
          }
        }
      }
    }
  }
  exports_names
}
