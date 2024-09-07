// presets.rs
mod react;
mod react_router;
mod react_router_dom;

use crate::parser::scan_exports::Import;

pub struct Preset {
  form: String,
  imports: Vec<String>,
}

pub fn resolve_presets(presets: &Vec<String>) -> Vec<Import> {
  let mut imports = Vec::new();
  for p in presets {
    let preset = match &p[..] {
        "react" => react::get_react_preset(),
        "react-router" => react_router::get_react_router_preset(),
        "react-router-dom" => react_router_dom::get_react_router_dom_preset(),
        _ => {
          println!("[farm-plugin-auto-import] Unknown preset: {}", p);
          continue;
        }
    };
    let form = preset.form;
    for import in &preset.imports {
      imports.push(Import {
        form: form.clone(),
        name: import.clone(),
        ..Default::default()
      });
    }
  }

  imports
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_resolve_presets() {
    let imports = resolve_presets(&vec!["react".to_string()]);
    println!("imports: {:#?}", imports)
  }
}
