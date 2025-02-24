#![deny(clippy::all)]

mod html_modifier;

pub use html_modifier::{inject_tags, Attrs, Tag};

use farmfe_core::{
  config::{config_regex::ConfigRegex, Config},
  context::CompilationContext,
  error::CompilationError,
  module::{module_graph::ModuleGraph, ModuleId, ModuleMetaData},
  plugin::{Plugin, PluginFinalizeResourcesHookParams, PluginProcessModuleHookParam},
  resource::{Resource, ResourceOrigin, ResourceType},
  swc_common::{
    comments::{Comments, SingleThreadedComments},
    Spanned,
  },
  swc_ecma_ast::{self, CallExpr, Callee, Expr, ExprOrSpread, Lit, Str},
};
use std::sync::{Arc, Mutex};

use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::common::PathFilter;
use farmfe_toolkit::swc_ecma_visit::{VisitMut, VisitMutWith};

// 存储预加载模块的信息
#[derive(Debug, Clone)]
struct PreloadModule {
  from_module_id: ModuleId,
  source: String,
  rel: String,
}

struct ImportVisitor<'a> {
  dynamic_imports: Vec<PreloadModule>,
  crossorigin: Option<bool>,
  comments: &'a SingleThreadedComments,
  module_id: ModuleId,
}

impl<'a> ImportVisitor<'a> {
  fn new(
    crossorigin: Option<bool>,
    comments: &'a SingleThreadedComments,
    module_id: ModuleId,
  ) -> Self {
    Self {
      dynamic_imports: Vec::new(),
      crossorigin,
      comments,
      module_id,
    }
  }

  fn handle_import_args(
    dynamic_imports: &mut Vec<PreloadModule>,
    args: &[ExprOrSpread],
    from_module_id: &ModuleId,
    comments: &SingleThreadedComments,
  ) {
    if let Some(arg) = args.first() {
      if let Some(comments) = comments.get_leading(arg.span_lo()) {
        if let Expr::Lit(Lit::Str(str_lit)) = &*arg.expr {
          let import_path = str_lit.value.to_string();
          for comment in comments {
            let comment_text = comment.text.trim();

            if comment_text.contains("prefetch: true") || comment_text.contains("preload: true") {
              dynamic_imports.push(PreloadModule {
                from_module_id: from_module_id.clone(),
                source: import_path.clone(),
                rel: "prefetch".to_string(),
              });
            }
          }
        }
      }
    }
  }

  fn check_import_call(
    dynamic_imports: &mut Vec<PreloadModule>,
    call: &CallExpr,
    from_module_id: &ModuleId,
    comments: &SingleThreadedComments,
  ) {
    match &call.callee {
      Callee::Import(_) => {
        Self::handle_import_args(dynamic_imports, &call.args, from_module_id, comments);
      }
      Callee::Expr(expr) => {
        if let Expr::Arrow(arrow) = &**expr {
          if let swc_ecma_ast::BlockStmtOrExpr::Expr(expr) = &*arrow.body {
            if let Expr::Call(inner_call) = &**expr {
              if let Callee::Import(_) = &inner_call.callee {
                Self::handle_import_args(
                  dynamic_imports,
                  &inner_call.args,
                  from_module_id,
                  comments,
                );
              }
            }
          }
        }
      }
      _ => {}
    }
  }
}

impl<'a> VisitMut for ImportVisitor<'a> {
  fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
    if let Some(comments) = self.comments.get_leading(call.span_lo()) {
      if comments
        .iter()
        .any(|comment| farmfe_utils::is_skip_action_by_comment(comment.text.as_str()))
      {
        return;
      }
    }

    Self::check_import_call(
      &mut self.dynamic_imports,
      &call,
      &self.module_id,
      self.comments,
    );
    call.visit_mut_children_with(self);
  }
}

#[farm_plugin]
pub struct FarmPluginMagicString {
  options: MagicStringOptions,
  preload_modules: Arc<Mutex<Vec<PreloadModule>>>,
}

#[derive(serde::Deserialize)]
pub struct MagicStringOptions {
  exclude: Vec<ConfigRegex>,
  include: Vec<ConfigRegex>,
  crossorigin: Option<bool>,
}

impl Default for MagicStringOptions {
  fn default() -> Self {
    Self {
      exclude: vec![ConfigRegex::new("node_modules/")],
      include: vec![ConfigRegex::new(".(js|ts|jsx|tsx)$")],
      crossorigin: Some(true),
    }
  }
}

impl FarmPluginMagicString {
  fn new(_: &Config, options: String) -> Self {
    let options: MagicStringOptions = serde_json::from_str(&options).unwrap_or_default();
    Self {
      options,
      preload_modules: Arc::new(Mutex::new(Vec::new())),
    }
  }
}

impl Plugin for FarmPluginMagicString {
  fn name(&self) -> &str {
    "FarmPluginMagicString"
  }

  fn process_module(
    &self,
    param: &mut PluginProcessModuleHookParam,
    _context: &Arc<CompilationContext>,
  ) -> Result<Option<()>, CompilationError> {
    let filter = PathFilter::new(&self.options.include, &self.options.exclude);
    if !filter.execute(param.module_id.relative_path()) {
      return Ok(None);
    }

    if !matches!(param.meta, ModuleMetaData::Script(_)) {
      return Ok(None);
    }

    let script = param.meta.as_script_mut();
    let comments: SingleThreadedComments = script.take_comments().into();
    let ast = &mut script.ast;
    let mut visitor =
      ImportVisitor::new(self.options.crossorigin, &comments, param.module_id.clone());
    ast.visit_mut_with(&mut visitor);

    if !visitor.dynamic_imports.is_empty() {
      if let Ok(mut modules) = self.preload_modules.lock() {
        modules.extend(visitor.dynamic_imports);
      }
    }

    Ok(Some(()))
  }

  fn finalize_resources(
    &self,
    params: &mut PluginFinalizeResourcesHookParams,
    context: &Arc<CompilationContext>,
  ) -> farmfe_core::error::Result<Option<()>> {
    let dynamic_imports = self.preload_modules.lock().unwrap();
    let mut tags = Vec::new();
    // let mut preload_urls = Vec::new();

    // 获取 module_graph 的读锁
    let module_graph = context.module_graph.read();
    let module_group_graph = context.module_group_graph.read();
    let resource_pot_map = context.resource_pot_map.read();
    // 处理每个动态导入
    for preload_module in dynamic_imports.iter() {
      // 使用 get_dep_by_source 获取依赖模块
      if let Some(dep_module_id) = module_graph.get_dep_by_source_optional(
        &preload_module.from_module_id, // 使用存储的源模块ID
        &preload_module.source,
        None,
      ) {
        // println!(
        // "preload_module: {:?}",
        // preload_module.from_module_id.relative_path()
        // );
        // println!("Found dep module: {}", dep_module_id.relative_path());
        // 在 resources_map 中查找对应的资源
        if let Some(module) = module_graph.module(&dep_module_id) {
          // 遍历 group 中的所有资源
          if let Some(group) = module_group_graph.module_group(&module.id) {
            // 获取排序后的 resource pots
            let sorted_pots = group.sorted_resource_pots(&module_graph, &resource_pot_map);

            // 遍历每个 resource pot
            for pot_id in sorted_pots {
              if let Some(pot) = resource_pot_map.resource_pot(&pot_id) {
                // 获取 pot 中的资源
                // for resource in pot.resources() {
                //   println!("Found resource in pot: {}", resource);
                // }
                for resource in pot.resources() {
                  // println!("Creating tag for resource: {}", resource);
                  // 创建 Tag 结构体
                  let tag = Tag {
                    tag: "link".to_string(),
                    inject_to: "head".to_string(),
                    attrs: Attrs {
                      rel: preload_module.rel.clone(),
                      crossorigin: self.options.crossorigin,
                      href: format!("/{}", resource),
                    },
                  };
                  tags.push(tag);
                }
              }
            }
          }
        }
      }
    }

    for (_, resource) in params.resources_map.iter_mut() {
      if matches!(resource.resource_type, ResourceType::Html) {
        let html_content = String::from_utf8_lossy(&resource.bytes);
        let new_html = match inject_tags(&html_content, tags.clone()) {
          Ok(html) => html,
          Err(e) => return Err(CompilationError::GenericError(e.to_string())),
        };
        // println!("new_html: {}", new_html);
        resource.bytes = new_html.into_bytes();
      }
    }
    // for (url, rel) in preload_urls.iter() {
    //   println!("Adding {} tag for: {}", rel, url);
    // }

    Ok(None)
  }
}
