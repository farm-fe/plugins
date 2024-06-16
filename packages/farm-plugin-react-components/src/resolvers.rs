use std::{collections::HashSet, fs, path::Path};

use crate::find_local_components::{ComponentInfo, ExportType};
use farmfe_core::{config::config_regex::ConfigRegex, regex::Regex, serde_json::Value};
use farmfe_toolkit::resolve::package_json_loader::{Options, PackageJsonLoader};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ResolverOption {
  pub module: String,
  pub prefix: Option<String>,
  pub export_type: Option<ExportType>,
  pub style: Option<bool>,
  pub exclude: Option<Vec<ConfigRegex>>,
  pub include: Option<Vec<ConfigRegex>>,
}

pub fn get_resolvers_result(
  root_path: &str,
  resolvers: Vec<ResolverOption>,
) -> HashSet<ComponentInfo> {
  let mut resolver_set = HashSet::new();
  for item in resolvers {
    let components = get_resolvers(root_path, item);
    for ele in components {
      resolver_set.insert(ele);
    }
  }
  resolver_set
}

pub fn get_resolvers(root_path: &str, component_lib: ResolverOption) -> Vec<ComponentInfo> {
  let mut components = vec![];
  let prefix = &component_lib.prefix.unwrap_or("".to_string());
  let loader = PackageJsonLoader::new();
  let package_path = Path::new(root_path).join(format!("node_modules/{}", &component_lib.module));
  let package_json = loader
    .load(
      package_path.clone(),
      Options {
        follow_symlinks: false,
        resolve_ancestor_dir: false,
      },
    )
    .unwrap();
  let default_relative_type_file = Value::from("./index.d.ts");
  let relative_type_file = package_json
    .raw_map()
    .get("typings")
    .unwrap_or(&default_relative_type_file)
    .as_str()
    .unwrap();
  print!("relative_type_file:{:?}", relative_type_file);
  let type_file = package_path.join(relative_type_file);
  println!("type_file:{:?}", type_file);
  let content = fs::read_to_string(type_file).expect("Failed to read file");
  let re = Regex::new(r"export\s+\{\s*default\s+as\s+(\w+)\s*\}\s+from\s+'\.\/(\w+)';").unwrap();
  for cap in re.captures_iter(&content) {
    components.push(ComponentInfo {
      name: format!("{}{}", prefix, cap[1].to_string()),
      path: component_lib.module.clone(),
      export_type: ExportType::Named,
      original_name: cap[1].to_string(),
      style: false,
    })
  }
  components
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;
  #[test]
  fn test_generate_dts() {
    let current_dir = env::current_dir().unwrap();
    let binding = current_dir.join("playground");
    let root_path = binding.to_str().unwrap();
    let resolver_option = ResolverOption {
      module: "antd".to_string(),
      export_type: Some(ExportType::Named),
      style: Some(false),
      exclude: None,
      include: None,
      prefix: Some("Ant".to_string()),
    };

    let components = get_resolvers(root_path, resolver_option);
    println!("components:{:#?}", components);
  }
}
