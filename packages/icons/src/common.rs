use farmfe_core::regex::{self, Regex};
use std::{
  fs::File,
  io::BufReader,
  path::{Path, PathBuf},
  process::Command,
};

pub const URL_PREFIXES: [&str; 4] = ["/~icons/", "~icons/", "virtual:icons/", "virtual/icons/"];

pub fn is_icon_path(path: &str) -> bool {
  let icon_path_re = regex::Regex::new(
    &URL_PREFIXES
      .iter()
      .map(|v| format!("^{}", v))
      .collect::<Vec<String>>()
      .join("|"),
  )
  .unwrap();
  icon_path_re.is_match(path)
}
#[derive(Debug)]
pub struct ResolveResult {
  pub collection: String,
  pub icon: String,
}

pub fn remove_prefix(path: &str) -> String {
  let mut path = path.to_string();
  for prefix in URL_PREFIXES.iter() {
    if path.starts_with(prefix) {
      path = path.replacen(prefix, "", 1);
      break;
    }
  }
  path
}

pub fn resolve_icons_path(path: &str) -> ResolveResult {
  let path = remove_prefix(path);
  let (path, _) = path.split_once(".").unzip();
  let (collection, icon) = path.unwrap().split_once("/").unzip();

  ResolveResult {
    collection: collection.unwrap().to_owned(),
    icon: icon.unwrap().to_owned(),
  }
}

pub struct PathMate {
  pub base_path: String,
  pub query: String,
}

pub fn get_icon_path_meta(path: &str) -> PathMate {
  let normalized_id = remove_prefix(path);
  let query_index = normalized_id.find('?').unwrap_or(normalized_id.len());

  let re_extension = Regex::new(r"\.\w+$").unwrap();
  let re_leading_slash = Regex::new(r"^/").unwrap();

  let base = if query_index < normalized_id.len() {
    &normalized_id[..query_index]
  } else {
    &normalized_id
  };

  let base = re_extension.replace(base, "").to_string();
  let base = re_leading_slash.replace(&base, "").to_string();

  let query = if query_index < normalized_id.len() {
    format!("?{}", &normalized_id[query_index + 1..])
  } else {
    "".to_string()
  };

  PathMate {
    base_path: base,
    query,
  }
}

#[derive(Debug)]
pub struct GetIconPathDataParams {
  pub path: String,
  pub project_dir: String,
  pub auto_install: bool,
}

pub fn get_icon_path_data(opt: GetIconPathDataParams) -> String {
  let resolved = resolve_icons_path(&opt.path);
  let all_icon_path = build_icon_path(&opt.project_dir, "@iconify/json/json");
  let icons_path = build_icon_path(
    &opt.project_dir,
    &format!("@iconify-json/{}", resolved.collection),
  );
  if !all_icon_path.exists() && !icons_path.exists() {
    if opt.auto_install {
      install_icon_package(&opt.project_dir, &resolved.collection);
    } else {
      return String::new();
    }
  }
  let icon_collection_path = all_icon_path.join(format!("{}.json", resolved.collection));
  if icon_collection_path.exists() {
    let json = read_json_from_file(icon_collection_path.to_str().unwrap());
    if let Some(body) = json
      .get("icons")
      .and_then(|icons| icons.get(&resolved.icon))
      .and_then(|icon| icon.get("body"))
    {
      return body.as_str().unwrap_or("").to_string();
    }
  }
  String::new()
}

fn read_json_from_file(file_path: &str) -> serde_json::Value {
  let file = File::open(file_path).expect("Failed to open file");
  let reader = BufReader::new(file);
  serde_json::from_reader(reader).expect("Failed to read JSON")
}

fn build_icon_path(project_dir: &str, sub_path: &str) -> PathBuf {
  Path::new(project_dir).join("node_modules").join(sub_path)
}

fn install_icon_package(project_dir: &str, collection: &str) {
  let pkg_manager = get_package_manager(project_dir);
  let cmd = match pkg_manager.as_str() {
    "npm" => format!("npm install @iconify-json/{}", collection),
    "pnpm" => format!("pnpm add @iconify-json/{}", collection),
    "yarn" => format!("yarn add @iconify-json/{}", collection),
    _ => panic!("Unknown package manager"),
  };
  let output = Command::new("sh")
    .arg("-c")
    .arg(cmd)
    .output()
    .expect("Failed to execute command");
  if !output.status.success() {
    panic!(
      "Command execution failed: {}",
      String::from_utf8_lossy(&output.stderr)
    );
  }
}

pub fn get_package_manager(project_dir: &str) -> String {
  find_package_manager_in_current_or_parent(Path::new(project_dir)).unwrap_or_else(|| "pnpm".to_string())
}

fn find_package_manager_in_current_or_parent(dir: &Path) -> Option<String> {
  if let Some(manager) = check_lock_files(dir) {
      return Some(manager);
  }

  if let Some(parent) = dir.parent() {
      return check_lock_files(parent);
  }

  None
}

fn check_lock_files(dir: &Path) -> Option<String> {
  let npm_lock = dir.join("package-lock.json");
  let pnpm_lock = dir.join("pnpm-lock.yaml");
  let yarn_lock = dir.join("yarn.lock");

  if npm_lock.exists() {
      Some("npm".to_string())
  } else if pnpm_lock.exists() {
      Some("pnpm".to_string())
  } else if yarn_lock.exists() {
      Some("yarn".to_string())
  } else {
      None
  }
}
