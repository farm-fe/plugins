use farmfe_core::regex::{Captures, Regex};
use farmfe_core::serde_json;
use farmfe_toolkit::hash::sha256;
use farmfe_toolkit::pluginutils::normalize_path::normalize_path;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashSet;
use walkdir::WalkDir;

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Route {
  path: String,
  index: bool,
  spread_module: Option<String>,
  children: Option<Vec<Route>>,
  lazy: Option<String>,
}

const ADAPTER_MODULE:&str = "function adapterModule(module) {
  const { default: Component, clientLoader: loader, clientAction: action, loader: _loader, action: _action, Component: _Component, ...other } = module;
  return { Component, loader, action, ...other };
}\n\n";

impl Serialize for Route {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("Route", 4)?;
    if !self.path.is_empty() {
      state.serialize_field("path", &self.path)?;
    }
    if self.index {
      state.serialize_field("index", &self.index)?;
    }
    if let Some(ref spread_module) = self.spread_module {
      state.serialize_field("spread_module", &format!("{}{}{}", "$", spread_module, "$"))?;
    }
    if let Some(ref lazy) = self.lazy {
      state.serialize_field("lazy", &format!("{}{}{}", "$", lazy, "$"))?;
    }
    if let Some(ref children) = self.children {
      state.serialize_field("children", children)?;
    }
    state.end()
  }
}

fn process_route_path(segment: &str) -> String {
  let patterns = [(r"\(([^)]*)\)\??$", "$1?"), (r"\$+$", "*"), (r"^\$", ":")];

  let mut segment = segment.to_string();
  for (pat, repl) in patterns.iter() {
    let re = Regex::new(pat).unwrap();
    segment = re.replace(&segment, *repl).into_owned();
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

    let page_condition = format!("{}{}", segment, suffix);
    let route_page_condition = format!("{}/route{}", segment, suffix);

    if let Some(page) = filtered_route_files.iter().find(|str: &&String| {
      let str_ext = str.split('.').last();
      let page_condition = format!("{}.{}", page_condition, str_ext.unwrap());
      let route_page_condition = format!("{}.{}", route_page_condition, str_ext.unwrap());
      str.ends_with(&page_condition) || str.ends_with(&route_page_condition)
    }) {
      let absolute_path = format!("{}/{}", routes_path, page);
      component = absolute_path;
      break;
    }
  }

  (component, is_lazy)
}

pub fn get_route_files(dir: &str) -> Vec<String> {
  let mut dir = dir.to_string();
  if !dir.ends_with('/') {
    dir.push_str("/");
  }
  WalkDir::new(&dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter_map(|e| {
      if e.path().is_file()
        && e
          .path()
          .extension()
          .map_or(false, |e| e == "jsx" || e == "tsx")
      {
        Some(normalize_path(e.path().to_str().unwrap()).replace(&dir, ""))
      } else {
        None
      }
    })
    .collect::<Vec<String>>()
}

pub fn parse(route_files: Vec<String>, routes_path: &str, level: usize) -> (Vec<Route>, String) {
  let mut routes: Vec<Route> = Vec::new();
  let mut imports = String::new();
  let first_segments: HashSet<_> = route_files
    .iter()
    .filter_map(|str| str.split('.').nth(level))
    .filter_map(|segment| match segment {
      "jsx" | "tsx" | "lazy" => None,
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
      route.index = true;
    } else if !route_path.starts_with('_') {
      route.path = route_path;
    }

    let (component_file_path, is_lazy) = process_page(&filtered_route_files, &segment, routes_path);
    if !component_file_path.is_empty() {
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
        route.spread_module = Some(format!("...adapterModule({})", import_name));
      } else {
        route.lazy = Some(format!(
          "() => import('{}').then(adapterModule)",
          component_file_path
        ));
      }
    }

    let (mut routes_map, ims) = parse(filtered_route_files, routes_path, level + 1);
    if !routes_map.is_empty() {
      imports.push_str(&ims);
      if segment.ends_with("_") {
        let real_segment = &segment[..segment.len() - 1];
        for mut route in routes_map.drain(..) {
          if !route.index {
            route.path = format!("{}/{}", real_segment, route.path);
          }
          routes.push(route);
        }
        route.children = Some(routes_map);
      } else {
        route.children = Some(routes_map);
        routes.push(route);
      }
    } else {
      routes.push(route);
    }
  }

  (routes, imports)
}

pub fn build_routes_virtual_code(routes: Vec<Route>, imports: String) -> String {
  let mut code = format!("{}\n\n{}", imports, ADAPTER_MODULE);

  let re = Regex::new(r#""spread_module": "\$(.*?)\$"|"\$(.*?)\$""#).expect("Invalid regex");

  let json_string = serde_json::to_string_pretty(&routes).expect("Failed to serialize routes");

  let json_string = re
    .replace_all(&json_string, |caps: &Captures| {
      caps
        .get(1)
        .or_else(|| caps.get(2))
        .map_or("", |m| m.as_str())
        .to_owned()
    })
    .into_owned();

  code.push_str("export const routes = ");
  code.push_str(&json_string);
  code.push_str(";\n");

  code
}
