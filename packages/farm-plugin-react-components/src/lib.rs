#![deny(clippy::all)]
#![feature(box_patterns)]
pub mod find_local_components;
pub mod generate_dts;
pub mod insert_import;
pub mod resolvers;

use std::{collections::HashSet, path::PathBuf, sync::Arc};

use farmfe_core::{
  config::{config_regex::ConfigRegex, Config},
  module::ModuleType,
  plugin::{Plugin, PluginTransformHookResult},
  serde_json,
  swc_ecma_parser::{Syntax, TsConfig},
};

use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::{
  common::{build_source_map, create_swc_source_map, PathFilter, Source},
  script::{codegen_module, parse_module, CodeGenCommentsConfig, ParseScriptModuleResult},
  swc_ecma_visit::VisitMutWith,
};
use find_local_components::{find_local_components, ComponentInfo};
use generate_dts::{generate_dts, GenerateDtsOption};
use insert_import::{ImportModifier, InsertImportModifier};
use resolvers::{get_resolvers_result, ResolverOption};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum ImportMode {
  Relative,
  Absolute,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Options {
  pub dirs: Option<Vec<ConfigRegex>>,
  pub dts: Option<bool>,
  pub local: Option<bool>,
  pub import_mode: Option<ImportMode>,
  pub include: Option<Vec<ConfigRegex>>,
  pub exclude: Option<Vec<ConfigRegex>>,
  pub resolvers: Option<Vec<ResolverOption>>,
}

#[derive(Debug)]
#[farm_plugin]
pub struct FarmPluginReactComponents {
  options: Options,
  components: HashSet<ComponentInfo>,
}

impl FarmPluginReactComponents {
  pub fn new(config: &Config, options: String) -> Self {
    let options: Options = serde_json::from_str(&options).unwrap();
    let mut components =
      find_local_components(&config.root, options.dirs.clone().unwrap_or(vec![]));
    let resolvers_components = get_resolvers_result(
      &config.root.clone(),
      options.resolvers.clone().unwrap_or(vec![]),
    );
    let local_components = components.clone();
    let generate_dts_option = GenerateDtsOption {
      root_path: config.root.clone(),
      components: &local_components.iter().collect::<Vec<_>>(),
      filename: "components.d.ts".to_string(),
      resolvers_components: &resolvers_components.iter().collect::<Vec<_>>(),
      local: options.local.unwrap_or(true),
    };
    let dts = options.dts.unwrap_or(true);
    if dts {
      generate_dts(generate_dts_option)
    }
    components.extend(resolvers_components);
    Self {
      options,
      components,
    }
  }
}

impl Plugin for FarmPluginReactComponents {
  fn name(&self) -> &str {
    "FarmPluginReactComponents"
  }

  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    if param.module_type != ModuleType::Jsx && param.module_type != ModuleType::Tsx {
      return Ok(None);
    }
    let options = self.options.clone();
    let include = options.include.unwrap_or(vec![]);
    let exclude = options.exclude.unwrap_or(vec![]);
    let filter = PathFilter::new(&include, &exclude);
    if !filter.execute(&param.module_id) {
      return Ok(None);
    }
    let (cm, _) = create_swc_source_map(Source {
      path: PathBuf::from(param.resolved_path),
      content: Arc::new(param.content.clone()),
    });
    let ParseScriptModuleResult { mut ast, comments } = match parse_module(
      &param.module_id,
      &param.content,
      Syntax::Typescript(TsConfig {
        tsx: true,
        decorators: false,
        dts: false,
        no_early_errors: true,
        disallow_ambiguous_jsx_like: true,
      }),
      context.config.script.target.clone(),
    ) {
      Ok(res) => res,
      Err(err) => {
        println!("{}", err.to_string());
        panic!("Parse {} failed. See error details above.", param.module_id);
      }
    };
    let components = self.components.clone();
    let mut import_modifier = ImportModifier::new(components);
    ast.visit_mut_with(&mut import_modifier);
    let used_components = import_modifier.used_components;
    let mut insert_import_modifier = InsertImportModifier::new(
      context.config.output.target_env.clone(),
      options.import_mode.unwrap_or(ImportMode::Absolute),
      param.resolved_path.to_owned(),
      used_components,
    );
    ast.visit_mut_with(&mut insert_import_modifier);
    let mut src_map = vec![];
    let transformed_content = codegen_module(
      &ast,
      context.config.script.target.clone(),
      cm.clone(),
      Some(&mut src_map),
      context.config.minify.enabled(),
      Some(CodeGenCommentsConfig {
        comments: &comments,
        config: &context.config.comments,
      }),
    )
    .unwrap();

    let output_code = String::from_utf8(transformed_content).unwrap();

    let map = {
      let map = build_source_map(cm, &src_map);
      let mut buf = vec![];
      map.to_writer(&mut buf).expect("failed to write sourcemap");
      Some(String::from_utf8(buf).unwrap())
    };

    Ok(Some(PluginTransformHookResult {
      content: output_code,
      source_map: map,
      module_type: Some(param.module_type.clone()),
      ignore_previous_source_map: false,
    }))
  }
}
