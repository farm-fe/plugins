use std::collections::HashMap;
use std::fs;
use std::path::Path;
use wasmparser::{Parser, Payload};

#[derive(Debug)]
pub struct WasmInfo {
  pub imports: Vec<ImportInfo>,
  pub exports: Vec<String>,
}

#[derive(Debug)]
pub struct ImportInfo {
  pub from: String,
  pub names: Vec<String>,
}

pub fn parse_wasm<P: AsRef<Path>>(wasm_file_path: P) -> Result<WasmInfo, String> {
  let wasm_binary = fs::read(wasm_file_path).map_err(|e| format!("Failed to read file: {}", e))?;
  let mut imports_map: HashMap<String, Vec<String>> = HashMap::default();
  let mut exports = Vec::new();

  let parser = Parser::new(0);
  for payload in parser.parse_all(&wasm_binary) {
    match payload.map_err(|e| format!("Failed to parse WASM: {}", e))? {
      Payload::ImportSection(imports) => {
        for import in imports {
          let import = import.map_err(|e| format!("Failed to read import: {}", e))?;
          imports_map
            .entry(import.module.to_string())
            .or_default()
            .push(import.name.to_string());
        }
      }
      Payload::ExportSection(exports_section) => {
        for export in exports_section {
          let export = export.map_err(|e| format!("Failed to read export: {}", e))?;
          exports.push(export.name.to_string());
        }
      }
      _ => {}
    }
  }

  let imports = imports_map
    .into_iter()
    .map(|(from, names)| ImportInfo { from, names })
    .collect();

  Ok(WasmInfo { imports, exports })
}

pub fn generate_glue_code<P: AsRef<Path>>(
  wasm_file_path: P,
  names: &GlueNames,
) -> Result<String, String> {
  let wasm_info = parse_wasm(wasm_file_path)?;

  let mut import_statements = Vec::new();
  let mut import_object_entries = Vec::new();

  for (i, import) in wasm_info.imports.iter().enumerate() {
    import_statements.push(format!(
      "import * as __farm__wasmImport_{} from {:?};",
      i, import.from
    ));

    let mut import_values = Vec::new();
    for name in &import.names {
      import_values.push(format!("{}: __farm__wasmImport_{}[{:?}]", name, i, name));
    }

    import_object_entries.push(format!(
      "{:?}: {{ {} }}",
      import.from,
      import_values.join(", ")
    ));
  }

  let init_code = format!(
    r#"const __farm__wasmModule = await {}({{{}}}, {});
    const __farm__wasmExports = __farm__wasmModule.exports;"#,
    names.init_wasm,
    import_object_entries.join(", "),
    names.wasm_url
  );

  let mut export_statements = Vec::new();
  for export in wasm_info.exports {
    if export == "default" {
      export_statements.push("export default __farm__wasmExports.default;".to_string());
    } else {
      export_statements.push(format!(
        "export const {} = __farm__wasmExports.{};",
        export, export
      ));
    }
  }

  let glue_code = [
    import_statements.join("\n"),
    init_code,
    export_statements.join("\n"),
  ]
  .join("\n");

  Ok(glue_code)
}

pub struct GlueNames {
  pub init_wasm: String,
  pub wasm_url: String,
}
