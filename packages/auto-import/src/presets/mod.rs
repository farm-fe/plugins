// presets.rs
mod react;
mod react_router;
mod react_router_dom;

use std::collections::HashMap;

use crate::parser::{parse::ExportType, scan_exports::Import};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ImportItem {
  String(String),
  Alias(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomPreset {
  #[serde(flatten)]
  pub imports: HashMap<String, Vec<ImportItem>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImportPreset {
  pub from: String,
  pub imports: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum PresetItem {
  String(String),
  Custom(CustomPreset),
  ImportPreset(ImportPreset),
}

pub struct Preset {
  form: String,
  import: String,
  alias: Option<String>,
}

pub fn parse_presets(presets: &Vec<PresetItem>) -> Vec<Preset> {
  let mut parsed_presets = Vec::new();
  for p in presets {
    match p {
      PresetItem::String(preset_name) => {
        let preset = match &preset_name[..] {
          "react" => react::get_react_preset(),
          "react-router" => react_router::get_react_router_preset(),
          "react-router-dom" => react_router_dom::get_react_router_dom_preset(),
          _ => {
            println!("[farm-plugin-auto-import] Unknown preset: {}", preset_name);
            continue;
          }
        };
        let form = preset.from;
        for import in &preset.imports {
          parsed_presets.push(Preset {
            form: form.clone(),
            import: import.clone(),
            alias: None,
          });
        }
      }
      PresetItem::Custom(custom_preset) => {
        for (form, imports) in &custom_preset.imports {
          for import in imports {
            match import {
              ImportItem::String(import) => {
                parsed_presets.push(Preset {
                  form: form.clone(),
                  import: import.clone(),
                  alias: None,
                });
              }
              ImportItem::Alias(aliases) => {
                parsed_presets.push(Preset {
                  form: form.clone(),
                  import: aliases[0].clone(),
                  alias: Some(aliases[1].clone()),
                });
              }
            }
          }
        }
      }
      PresetItem::ImportPreset(import_preset) => {
        let form = import_preset.from.clone();
        for import in &import_preset.imports {
          parsed_presets.push(Preset {
            form: form.clone(),
            import: import.clone(),
            alias: None,
          });
        }
      }
    }
  }
  parsed_presets
}
pub fn resolve_presets(presets: &Vec<PresetItem>) -> Vec<Import> {
  let parsed_presets = parse_presets(presets);
  let mut imports = Vec::new();
  for p in parsed_presets {
    if p.alias.is_some() {
      let import = Import {
        form: p.form,
        name: p.import,
        priority: 0,
        export_type: ExportType::Namespace,
        as_name: p.alias,
        ..Default::default()
      };
      imports.push(import);
      continue;
    } else {
      let import = Import {
        form: p.form,
        name: p.import,
        priority: 0,
        export_type: ExportType::Declaration,
        ..Default::default()
      };
      imports.push(import);
    }
  }
  imports
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_resolve_presets() {
//     let imports = resolve_presets(&vec![PresetItem::Preset(Preset::String(
//       "react".to_string(),
//     ))]);
//     println!("imports: {:#?}", imports)
//   }
// }
