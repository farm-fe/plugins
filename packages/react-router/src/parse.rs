use std::fs;
use std::path::{Path, PathBuf};

enum Mark {
  Clint,
  Server,
}

fn file_sign_default() -> FileSign {
  Mark::Clint
}

#[derive(Debug)]
struct Route {
  index: Option<bool>,
  path: String,
  component: Option<String>,
  error_boundary: Option<String>,
  
  lazy: Option<bool>,
  #[serde(file_sign_default)]
  mark: Option<Mark>,
  children: Vec<Route>,
}

fn scan_routes(dir: &Path, base_path: &str) -> Vec<Route> {
  let mut routes = Vec::new();
  if let Ok(entries) = fs::read_dir(dir) {
    for entry in entries {
      if let Ok(entry) = entry {
        let path = entry.path();
        let file_name = entry.file_name().into_string().unwrap();
        let route_path = if base_path.is_empty() {
          file_name.clone()
        } else {
          format!("{}/{}", base_path, file_name)
        };

        if path.is_dir() {
          let children = scan_routes(&path, &route_path);
          routes.push(Route {
            path: parse_route_path(&route_path),
            component: file_name,
            children,
          });
        } else if path.is_file() && file_name.ends_with(".tsx") {
          routes.push(Route {
            path: parse_route_path(&route_path),
            component: file_name.replace(".tsx", ""),
            children: Vec::new(),
          });
        }
      }
    }
  }
  routes
}

fn parse_route_path(file_path: &str) -> String {
  file_path
    .replace("\\", "/")
    .replace("/index.tsx", "/")
    .replace(".tsx", "")
    .replace("[", ":")
    .replace("]", "")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_route_path() {
    let routes_dir = Path::new(
      "/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-components/playground/src/routes",
    );
    let routes = scan_routes(routes_dir, "");
    for route in routes {
      println!("{:?}", route);
    }
  }
}
