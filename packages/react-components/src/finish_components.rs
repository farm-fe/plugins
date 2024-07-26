use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use farmfe_core::config::config_regex::ConfigRegex;

use crate::find_local_components::{find_local_components, ComponentInfo};
use crate::generate_dts::{generate_dts, GenerateDtsOption};
use crate::resolvers::{get_resolvers_result, ResolverOption};

pub struct FinishComponentsParams<'a> {
  pub root_path: String,
  pub resolvers: Vec<ResolverOption>,
  pub dirs: Vec<ConfigRegex>,
  pub filename: String,
  pub local: bool,
  pub dts: bool,
  pub context_components: &'a Arc<Mutex<HashSet<ComponentInfo>>>,
}

pub struct FinishComponentsResult {
  pub local_components: HashSet<ComponentInfo>,
  pub resolvers_components: HashSet<ComponentInfo>,
}

fn has_new_or_removed_components(
  old_components: &HashSet<ComponentInfo>,
  local_components: &HashSet<ComponentInfo>,
  resolvers_components: &HashSet<ComponentInfo>,
) -> bool {
  let old_len = old_components.len();
  let new_len = local_components.len() + resolvers_components.len();
  old_len != new_len
    || local_components.iter().any(|component| !old_components.contains(component))
    || resolvers_components.iter().any(|component| !old_components.contains(component))
}

pub fn finish_components(params: FinishComponentsParams) {
  let FinishComponentsParams {
    root_path,
    resolvers,
    dirs,
    filename,
    local,
    dts,
    context_components,
  } = params;

  let mut local_components = find_local_components(&root_path, dirs);
  let resolvers_components = get_resolvers_result(&root_path, resolvers);
  let mut context_components_guard = match context_components.lock() {
    Ok(guard) => guard,
    Err(poisoned) => poisoned.into_inner(),
  };

  if has_new_or_removed_components(&context_components_guard, &local_components, &resolvers_components) && dts {
    let generate_dts_option = GenerateDtsOption {
      filename,
      root_path,
      local,
      components: &local_components.iter().collect::<Vec<_>>(),
      resolvers_components: &resolvers_components.iter().collect::<Vec<_>>(),
    };
    generate_dts(generate_dts_option);
    local_components.extend(resolvers_components);
    *context_components_guard = local_components;
  }
}
