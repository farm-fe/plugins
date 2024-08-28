use farmfe_toolkit::pluginutils::normalize_path::normalize_path;
use std::{collections::HashMap, path::Path};
use walkdir::WalkDir;

#[derive(Debug)]
enum Mark {
  Clint,
  Server,
}

#[derive(Default, Debug)]
struct Route {
  index: Option<bool>,
  path: String,
  component: String,
  error_boundary: Option<String>,
  lazy: Option<bool>,
  mark: Option<Mark>,
  children: Vec<Route>,
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

fn parse_routes(segments: Vec<String>, level: usize) -> Vec<Route> {
  let mut result = Vec::new();
  let mut route_map: HashMap<String, Route> = HashMap::new();

  for segment in segments {
    let filtered_strings: Vec<String> = segments
      .iter()
      .filter(|s| {
        let parts: Vec<&str> = s.split('.').collect();
        parts.len() > level
          && (parts[level] == segment || parts[level] == format!("{}/route", segment))
      })
      .collect();

    if filtered_strings.len() < 1 {
      continue;
    }

    let route_path = segment
      .replace(r"\(([^)]*)\)\??$", "$1?")
      .replace(r"\$+$", "*")
      .replace(r"^\$", ":");

    let is_lazy = segment.ends_with(".lazy.tsx") || segment.ends_with(".lazy.route.tsx");
    let mut new_node = Route {
      path: route_path.clone(),
      index: Some(false),
      component: format!("{}{}", route_path, ".tsx"),
      lazy: Some(is_lazy),
      ..Default::default()
    };

    if route_path == "_index" {
      new_node.index = Some(true);
    } else if !route_path.starts_with('_') {
      new_node.path = route_path;
    }

    let children = parse_routes(filtered_strings.clone(), level + 1);
    if children.len() > 0 {
      new_node.children = children;
    }
    for (_, route) in route_map {
      result.push(route);
    }

    return result;
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_route_path() {
    let routes_dir = Path::new(
      "/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/",
    );

    let files = get_route_files(routes_dir);
    // 处理一下 files 的路径 home.a.b.lazy.tsx -> [home, a, b, lazy]

    // let enters = files
    //   .iter()
    //   .map(|f| f.split('.').collect::<Vec<&str>>())
    //   .collect::<Vec<Vec<&str>>>();

    let routes = parse_routes(files, 0);
    // 有格式的输出
    println!("{:#?}", routes);
  }
}
