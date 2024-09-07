use super::scan_exports::Import;
use farmfe_core::serde_json::from_str;
use serde::Deserialize;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
#[derive(Deserialize)]
struct Preset {
  form: String,
  imports: Vec<String>,
}

pub fn resolve_presets(presets: &[&str]) -> Result<Vec<Import>, Box<dyn Error>> {
  let mut imports = Vec::new();
  let current_dir = std::env::current_dir().unwrap();
  println!("current_dir: {:?}", current_dir);
  for &preset in presets {
    let preset_path = Path::new("../presets").join(format!("{}.json", preset));
    if !preset_path.exists() {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("Preset file not found: {:?}", preset_path),
      )));
    }

    let content = read_to_string(&preset_path)?;
    let preset: Preset = from_str(&content)?;
    let form = preset.form;

    for import in &preset.imports {
      imports.push(Import {
        form: form.clone(),
        name: import.clone(),
        ..Default::default()
      });
    }
  }

  Ok(imports)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_resolve_presets() {
    let imports = resolve_presets(&vec!["react"]);
    println!("imports: {:#?}", imports)
  }
}
