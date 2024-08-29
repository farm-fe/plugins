use farmfe_core::regex::Regex;
use farmfe_core::serde_json;
use farmfe_toolkit::hash::sha256;
use farmfe_toolkit::pluginutils::normalize_path::normalize_path;
use serde::de::{self, EnumAccess, MapAccess, VariantAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::{collections::HashSet, path::Path};
use walkdir::WalkDir;

#[derive(Default, Debug, Deserialize, Clone)]
struct Route {
  index: Option<bool>,
  path: String,
  component: String,
  children: Option<Vec<Route>>,
}

impl Serialize for Route {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("Route", 4)?;
    if let Some(ref index) = self.index {
      state.serialize_field("index", index)?;
    }
    if let Some(ref children) = self.children {
      state.serialize_field("children", children)?;
    }
    state.serialize_field("path", &self.path)?;
    state.serialize_field("component", &format!("{}{}{}", "$", &self.component, "$"))?;
    state.end()
  }
}

fn process_route_path(segment: &str) -> String {
  let patterns = [(r"\(([^)]*)\)\??$", "$1?"), (r"\$+$", "*"), (r"^\$", ":")];

  let mut segment = segment.to_string();
  for (pat, repl) in patterns.iter() {
    let re = Regex::new(pat).unwrap();
    segment = re.replace_all(&segment, *repl).into_owned();
  }

  segment
}

fn process_page(
  filtered_route_files: &[String],
  segment: &str,
  routes_path: &str,
) -> (String, bool) {
  let mut component = String::new();
  let mut is_lazy = false;
  for page_type in &[("", false), (".lazy", true)] {
    let suffix = page_type.0;
    is_lazy = page_type.1;

    let page_condition = format!("{}{}.tsx", segment, suffix);
    let route_page_condition = format!("{}/route{}.tsx", segment, suffix);

    if let Some(page) = filtered_route_files
      .iter()
      .find(|str| str.ends_with(&page_condition) || str.ends_with(&route_page_condition))
    {
      let absolute_path = format!("{}/{}", routes_path, page);
      component = absolute_path;
      break;
    }
  }

  (component, is_lazy)
}

fn get_route_files(dir: &Path) -> Vec<String> {
  let dir_str = dir.to_str().unwrap();
  WalkDir::new(dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter_map(|e| {
      if e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "tsx") {
        Some(normalize_path(e.path().to_str().unwrap()).replace(dir_str, ""))
      } else {
        None
      }
    })
    .collect::<Vec<String>>()
}

fn parse(route_files: Vec<String>, routes_path: &str, level: usize) -> (Vec<Route>, String) {
  let mut routes: Vec<Route> = Vec::new();
  let mut imports = String::new();
  let first_segments: HashSet<_> = route_files
    .iter()
    .filter_map(|str| str.split('.').nth(level))
    .filter_map(|segment| match segment {
      "tsx" | "lazy" => None,
      _ => Some(segment.replace("/route", "")),
    })
    .collect();
  if first_segments.is_empty() {
    return (routes, imports);
  }

  let reversed_segments: Vec<_> = first_segments.into_iter().collect();

  for segment in reversed_segments.into_iter().rev() {
    let filtered_route_files: Vec<_> = route_files
      .iter()
      .filter(|str| {
        str
          .split('.')
          .nth(level)
          .map_or(false, |s| s == segment || s == format!("{}/route", segment))
      })
      .cloned()
      .collect();

    let route_path = process_route_path(&segment);
    if filtered_route_files.is_empty() {
      continue;
    }

    let mut route = Route::default();

    if route_path == "_index" {
      route.index = Some(true);
    } else if !route_path.starts_with('_') {
      route.path = route_path;
    }

    let (component_file_path, is_lazy) = process_page(&filtered_route_files, &segment, routes_path);

    if !is_lazy {
      let import_name = format!(
        "{}{}",
        "farmfe_plugin_react_router_",
        sha256(&component_file_path.as_bytes(), 8)
      );
      imports.push_str(&format!(
        "import * as {} from '{}';\n",
        import_name, component_file_path
      ));
      route.component = format!("...adapter({})", import_name);
    } else {
      route.component = format!(
        "lazy(() => import('{}').then(adapter))",
        component_file_path
      );
    }

    let (routes_map, imps) = parse(filtered_route_files, routes_path, level + 1);
    if !routes_map.is_empty() {
      route.children = Some(routes_map);
      imports.push_str(&imps);
    }
    routes.push(route);
  }

  (routes, imports)
}

fn build_routes_virtual_code(routes: Vec<Route>, imports: String) -> String {
  let mut code = String::new();
  code.push_str(&imports);
  code.push_str("\n\n");
  code.push_str("const routes = ");
  let re = Regex::new(r"\$(.*?)\$").unwrap();
  let json_string = serde_json::to_string_pretty(&routes)
    .unwrap();
  let json_string = re.replace_all(&json_string, "$1").into_owned();
  code.push_str(&json_string);
  code.push_str(";\n");
  code
}

#[cfg(test)]
mod tests {
  use std::fs;

  use super::*;

  #[test]
  fn test_parse_route_path() {
    let routes_dir = Path::new(
      "/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/",
    );

    let files = get_route_files(routes_dir);

    let (routes, imports) = parse(
      files,
      "/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes",
      0,
    );
    let code = build_routes_virtual_code(routes, imports);
    fs::write("/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/feat/plugin_code.ts", code);
  }

  #[test]
  fn test_process_route_path() {
    assert_eq!(process_route_path("user(id)"), "userid?");
    assert_eq!(process_route_path("find$"), "find*");
    assert_eq!(process_route_path("$edit"), ":edit");
    assert_eq!(process_route_path("user(id)$"), "user(id)*");
    assert_eq!(process_route_path("$(id)"), ":id?");
  }
}
