use std::collections::HashSet;

use farmfe_core::config::config_regex::ConfigRegex;

use crate::find_local_components::{find_local_components, ComponentInfo};
use crate::generate_dts::generate_dts;
use crate::generate_dts::GenerateDtsOption;
use crate::resolvers::{get_resolvers_result, ResolverOption};

pub struct FinishComponentsParams {
  pub root_path: String,
  pub resolvers: Vec<ResolverOption>,
  pub dirs: Vec<ConfigRegex>,
  pub filename: String,
  pub local: bool,
  pub dts: bool,
}

pub fn finish_components(params: FinishComponentsParams) -> HashSet<ComponentInfo> {
  let FinishComponentsParams {
    root_path,
    resolvers,
    dirs,
    filename,
    local,
    dts,
  } = params;
  let mut local_components = find_local_components(&root_path, dirs);
  let resolvers_components = get_resolvers_result(&root_path, resolvers);
  let generate_dts_option = GenerateDtsOption {
    filename,
    root_path: root_path.clone(),
    components: &local_components.iter().collect::<Vec<_>>(),
    resolvers_components: &resolvers_components.iter().collect::<Vec<_>>(),
    local,
  };
  if dts {
    generate_dts(generate_dts_option);
  }
  local_components.extend(resolvers_components);
  local_components
}
