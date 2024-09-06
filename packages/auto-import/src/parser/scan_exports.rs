use std::path::Path;

use crate::parser::parse::{
  parse_esm_imports_exports, DeclarationType, ESMExport, ESMImport, ExportType,
};

pub struct Export {
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

pub fn scan_exports(file_path: &str, content: &str) -> Vec<Export> {
  let (_, exports) = parse_esm_imports_exports(None, Some(content));
  let filename = get_filename_by_path(file_path);
  let mut exports_names = Vec::new();
  for export in exports {
    let ESMExport {
      name,
      declaration_type,
      default_name,
      named_exports,
      specifier,
      export_type,
      type_named_exports,
    } = export;
    match export_type {
      ExportType::Default => {
        exports_names.push(Export {
          form: "export default".to_string(),
          name: default_name.unwrap(),
          priority: 0,
          disabled: None,
          dts_disabled: None,
          declaration_type: None,
          tp: None,
          type_from: None,
          as_name: None,
        });
      },
      // do it later
      _ =>{}
    }
  }
  exports_names
}
