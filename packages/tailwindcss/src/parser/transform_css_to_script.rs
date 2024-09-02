use std::{path::PathBuf, string, sync::Arc};
use tailwind_css::TailwindBuilder;

use farmfe_core::{
  cache::cache_store::CacheStoreKey,
  context::CompilationContext,
  deserialize,
  enhanced_magic_string::collapse_sourcemap::{collapse_sourcemap_chain, CollapseSourcemapOptions},
  module::{
    CommentsMetaData, CssModuleMetaData, ModuleId, ModuleMetaData, ModuleSystem, ModuleType,
    ScriptModuleMetaData,
  },
  plugin::ResolveKind,
  rayon::prelude::*,
  serialize,
  swc_common::Mark,
  swc_css_ast::Stylesheet,
  swc_ecma_ast::EsVersion,
  swc_ecma_parser::Syntax,
};
use farmfe_toolkit::{
  common::{create_swc_source_map, Source},
  css::{parse_css_stylesheet, ParseCssModuleResult},
  hash::base64_encode,
  script::{parse_module, swc_try_with::try_with, ParseScriptModuleResult},
  sourcemap::SourceMap,
  swc_ecma_transforms_base::resolver,
  swc_ecma_visit::VisitMutWith,
};
use farmfe_utils::{hash::sha256, relative};

pub fn transform_css_to_script_modules(
  id: &str,
  tw_code: String,
  module_ids: Vec<ModuleId>,
  context: &Arc<CompilationContext>,
) -> farmfe_core::error::Result<()> {
  module_ids
    .into_par_iter()
    .filter(|mid| mid.to_string() == id)
    .try_for_each(|module_id: ModuleId| {
      let css_code = Arc::new(tw_code.clone());
      println!(
        "[FarmfePluginTailwindcss] transform css to script: {:#?}",
        tw_code
      );
      let (cm, _) = create_swc_source_map(Source {
        path: PathBuf::from(module_id.to_string()),
        content: css_code.clone(),
      });
      {
        context
          .module_graph
          .write()
          .module_mut(&module_id)
          .unwrap()
          .content = css_code.clone();
      }

      try_with(cm.clone(), &context.meta.script.globals, || {
        let ParseCssModuleResult { ast, comments } =
          parse_css_stylesheet(&module_id.to_string(), css_code).unwrap();
        let mut module_graph = context.module_graph.write();
        let module = module_graph.module_mut(&module_id).unwrap();

        module.meta = Box::new(ModuleMetaData::Css(CssModuleMetaData {
          ast,
          comments: CommentsMetaData::from(comments),
          custom: Default::default(),
        }));

        module.module_type = ModuleType::Css;
      })
    })
}
