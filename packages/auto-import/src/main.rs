use farmfe_core::serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ImportItem {
    String(String),
    Alias(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomPreset {
    #[serde(flatten)]
    pub imports: HashMap<String, Vec<ImportItem>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImportPreset {
    pub from: String,
    pub imports: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PresetItem {
    String(String),
    Custom(CustomPreset),
    ImportPreset(ImportPreset),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub presets: Vec<PresetItem>,
}

fn main() {
    let json_data = r#"
    [
        "react",
        "react-router",
        {
          "@vueuse/core": [
            "useMouse",
            ["useFetch", "useMyFetch"],
            ["useStorage", "useMyStorage"]
          ]
        },
        {
          "from": "vue-router",
          "imports": ["RouteLocationRaw"]
        }
    ]
    "#;

    let config: Vec<PresetItem> = serde_json::from_str(json_data).unwrap();
    println!("{:#?}", config);
}
