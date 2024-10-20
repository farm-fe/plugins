#![deny(clippy::all)]

use std::path::PathBuf;
use std::sync::Arc;

use farmfe_core::plugin::PluginTransformHookResult;
use farmfe_core::serde_json;
use farmfe_core::swc_common::comments::SingleThreadedComments;
use farmfe_core::{config::Config, plugin::Plugin};

use farmfe_macro_plugin::farm_plugin;
use vue_jsx_visitor::Options;
use vue_jsx_visitor::VueJsxTransformVisitor;

use farmfe_core::{
  module::ModuleType,
  swc_common::Mark,
  swc_ecma_parser::{Syntax, TsSyntax},
};

use farmfe_toolkit::{
  common::{build_source_map, create_swc_source_map, Source},
  script::{codegen_module, parse_module, ParseScriptModuleResult},
  swc_ecma_transforms_base::resolver,
  swc_ecma_visit::VisitMutWith,
};

#[farm_plugin]
pub struct FarmfePluginVueJsx {
  options: Options,
}

impl FarmfePluginVueJsx {
  fn new(_config: &Config, options: String) -> Self {
    let options: Options = serde_json::from_str(&options).unwrap();
    Self { options }
  }
}

impl Plugin for FarmfePluginVueJsx {
  fn name(&self) -> &str {
    "FarmfePluginVueJsx"
  }
  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    if [ModuleType::Jsx, ModuleType::Tsx].contains(&param.module_type) {
      println!("module_id: {}", param.module_id);
      let options = self.options.clone();
      let (cm, _) = create_swc_source_map(Source {
        path: PathBuf::from(param.resolved_path),
        content: Arc::new(param.content.clone()),
      });
      let unresolved_mark = Mark::new();
      let top_level_mark = Mark::new();
      let ParseScriptModuleResult { mut ast, comments } = match parse_module(
        &param.module_id,
        &param.content,
        Syntax::Typescript(TsSyntax {
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
      ast.visit_mut_with(&mut resolver(unresolved_mark, top_level_mark, true));
      let mut vis: VueJsxTransformVisitor<SingleThreadedComments> =
        VueJsxTransformVisitor::new(options, unresolved_mark, Some(comments));
      ast.visit_mut_with(&mut vis);
      let mut src_map = vec![];
      let transformed_content = codegen_module(
        &ast,
        context.config.script.target.clone(),
        cm.clone(),
        Some(&mut src_map),
        context.config.minify.enabled(),
        None,
      )
      .unwrap();

      let output_code = String::from_utf8(transformed_content).unwrap();
      println!("{}", output_code);
      let map = {
        let map = build_source_map(cm, &src_map);
        let mut buf = vec![];
        map.to_writer(&mut buf).expect("failed to write sourcemap");
        Some(String::from_utf8(buf).unwrap())
      };

      Ok(Some(PluginTransformHookResult {
        content: output_code,
        source_map: map,
        module_type: Some(ModuleType::Js),
        ignore_previous_source_map: false,
      }))
    } else {
      return Ok(None);
    }
  }
}
