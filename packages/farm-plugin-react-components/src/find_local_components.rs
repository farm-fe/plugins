use crate::resolvers::ImportStyle;
use farmfe_core::{
  config::config_regex::ConfigRegex,
  swc_ecma_ast::*,
  swc_ecma_parser::{Syntax, TsConfig},
};
use farmfe_toolkit::{
  common::PathFilter,
  script::{parse_module, ParseScriptModuleResult},
  swc_ecma_visit::{Visit, VisitWith},
};
use glob::Pattern;
use std::fs;
use std::path::PathBuf;
use std::{collections::HashSet, path::Path};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct ComponentInfo {
  pub name: String,
  pub path: String,
  pub export_type: ExportType,
  pub original_name: String,
  pub import_style: ImportStyle,
  pub is_local: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Hash, Eq, PartialEq, Clone)]
pub enum ExportType {
  Named,
  Default,
}
#[derive(Clone)]
pub struct ComponentFinder {
  all_components: Vec<String>,
  #[allow(dead_code)]
  filename: Option<String>,
  #[allow(dead_code)]
  file_path: String,
}

#[derive(Clone)]
pub struct ExportComponentsFinder {
  exported_components: HashSet<ComponentInfo>,
  all_components: Vec<String>,
  filename: Option<String>,
  path: String,
}
fn is_jsx_return_with_block_stmt(body: &Option<BlockStmt>) -> bool {
  if let Some(body) = body {
    body.stmts.iter().any(|stmt| {
      matches!(
        stmt,
        Stmt::Return(ReturnStmt {
          arg: Some(box Expr::JSXElement(..)),
          ..
        })
      )
    })
  } else {
    false
  }
}
fn is_jsx_return_with_block_stmt_or_expr(body: &BlockStmtOrExpr) -> bool {
  match body {
    BlockStmtOrExpr::BlockStmt(block) => block.stmts.iter().any(|stmt| {
      matches!(
        stmt,
        Stmt::Return(ReturnStmt {
          arg: Some(box Expr::JSXElement(..)),
          ..
        })
      )
    }),
    BlockStmtOrExpr::Expr(box Expr::JSXElement(..)) => true,
    _ => false,
  }
}

impl ExportComponentsFinder {
  fn new(path: &str, all_components: &Vec<String>) -> Self {
    Self {
      path: path.to_owned(),
      filename: get_filename_by_path(path),
      exported_components: HashSet::new(),
      all_components: all_components.to_vec(),
    }
  }

  fn is_component(&self, name: &String) -> bool {
    self.all_components.contains(name)
  }

  fn add_exported_components(&mut self, name: &str, export_type: ExportType) {
    self.exported_components.insert(ComponentInfo {
      name: name.to_string(),
      path: self.path.clone(),
      export_type,
      original_name: name.to_string(),
      import_style: ImportStyle::Bool(false),
      is_local: true,
    });
  }
}

fn get_filename_by_path(file_path: &str) -> Option<String> {
  let path = Path::new(file_path);
  path
    .file_stem()
    .and_then(|filename_osstr| filename_osstr.to_str())
    .map(|filename_str| filename_str.to_owned())
}

impl ComponentFinder {
  fn new(file_path: &str) -> Self {
    Self {
      all_components: vec![],
      file_path: file_path.to_owned(),
      filename: get_filename_by_path(file_path),
    }
  }

  fn add_component(&mut self, name: &str) {
    self.all_components.push(name.to_owned())
  }
}

impl Visit for ComponentFinder {
  fn visit_var_decl(&mut self, var_decl: &VarDecl) {
    for decl in &var_decl.decls {
      if let Some(init) = &decl.init {
        match &**init {
          Expr::Arrow(arrow_expr) => {
            if is_jsx_return_with_block_stmt_or_expr(&arrow_expr.body) {
              if let Pat::Ident(ident) = &decl.name {
                self.add_component(&ident.id.sym.to_string())
              }
            }
          }
          Expr::Fn(fn_expr) => {
            if is_jsx_return_with_block_stmt(&fn_expr.function.body) {
              if let Pat::Ident(ident) = &decl.name {
                self.add_component(&ident.id.sym.to_string())
              }
            }
          }
          _ => {}
        }
      }
    }
  }

  fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
    if is_jsx_return_with_block_stmt(&fn_decl.function.body) {
      self.add_component(&fn_decl.ident.sym.to_string());
    }
  }
}

impl Visit for ExportComponentsFinder {
  fn visit_export_decl(&mut self, n: &ExportDecl) {
    match &n.decl {
      Decl::Fn(fn_decl) => {
        if is_jsx_return_with_block_stmt(&fn_decl.function.body) {
          let sym = &fn_decl.ident.sym;
          self.add_exported_components(&sym.to_string(), ExportType::Named);
        }
      }
      // export const MyComponent1 = () => <div />;
      // export const MyComponent3 = function(){return <div />}
      // function MyComponent2(){return <div />}
      // export{ MyComponent2 }
      Decl::Var(var_decl) => {
        for var in &var_decl.decls {
          if let Pat::Ident(var_ident) = &var.name {
            if let Some(init_expr) = &var.init {
              match &**init_expr {
                Expr::Arrow(arrow_expr) => {
                  if is_jsx_return_with_block_stmt_or_expr(&arrow_expr.body) {
                    self.add_exported_components(&var_ident.id.sym.to_string(), ExportType::Named);
                  }
                }
                Expr::Fn(fn_expr) => {
                  if is_jsx_return_with_block_stmt(&fn_expr.function.body) {
                    self.add_exported_components(&var_ident.id.sym.to_string(), ExportType::Named);
                  }
                }
                _ => {}
              }
            }
          }
        }
      }
      _ => {}
    }
    n.visit_children_with(self);
  }

  fn visit_named_export(&mut self, export_named: &NamedExport) {
    for specifier in &export_named.specifiers {
      match specifier {
        ExportSpecifier::Named(named) => {
          if let ModuleExportName::Ident(name) = named.exported.as_ref().unwrap_or(&named.orig) {
            let component_name = name.sym.to_string();
            if self.is_component(&component_name) {
              self.add_exported_components(&component_name, ExportType::Named)
            }
          };
        }
        _ => {}
      }
    }
  }

  fn visit_export_default_decl(&mut self, export_default: &ExportDefaultDecl) {
    let component_name = self.filename.clone().unwrap_or("default".to_owned());
    match &export_default.decl {
      DefaultDecl::Fn(fn_dec) => {
        if is_jsx_return_with_block_stmt(&fn_dec.function.body) {
          let name = fn_dec
            .ident
            .as_ref()
            .map_or(component_name, |ident| ident.sym.to_string());
          self.add_exported_components(&name, ExportType::Default);
        }
      }
      _ => {}
    }
    export_default.visit_children_with(self);
  }
  // export default MyComponent
  // export default ()=>{return <div/>}
  // export default function(){return <div/>}
  fn visit_export_default_expr(&mut self, n: &ExportDefaultExpr) {
    let component_name = self.filename.clone().unwrap_or("default".to_owned());
    match &*n.expr {
      // 处理 export default ()=>{return <div/>}
      Expr::Arrow(arrow_expr) => {
        if is_jsx_return_with_block_stmt_or_expr(&arrow_expr.body) {
          self.add_exported_components(&component_name, ExportType::Default);
        }
      }
      // 处理 export default function(){return <div/>}
      Expr::Fn(fn_expr) => {
        if is_jsx_return_with_block_stmt(&fn_expr.function.body) {
          self.add_exported_components(&component_name, ExportType::Default);
        }
      }
      Expr::Ident(ident) => {
        let component_name = ident.sym.to_string();
        if self.is_component(&component_name) {
          self.add_exported_components(&component_name, ExportType::Named)
        }
      }
      _ => {}
    }
  }
}

// Function to parse the code in a .tsx/.jsx file and collect React components
fn gen_components_by_file(file_path: &PathBuf) -> HashSet<ComponentInfo> {
  let file_content = fs::read_to_string(file_path)
    .unwrap_or_else(|_| panic!("Unable to read file: {:?}", file_path));
  let components_path = file_path.to_string_lossy().into_owned();
  let ParseScriptModuleResult { ast, comments: _ } = match parse_module(
    &components_path,
    &file_content,
    Syntax::Typescript(TsConfig {
      tsx: true,
      decorators: true,
      ..Default::default()
    }),
    EsVersion::latest(),
  ) {
    Ok(res) => res,
    Err(err) => {
      println!("{}", err.to_string());
      panic!("Parse {} failed. See error details above.", components_path);
    }
  };
  let mut components_finder = ComponentFinder::new(&components_path);
  ast.visit_with(&mut components_finder);
  let all_components = components_finder.all_components;
  let mut export_components_finder = ExportComponentsFinder::new(&components_path, &all_components);
  ast.visit_with(&mut export_components_finder);
  let export_components = export_components_finder.exported_components;
  export_components
}

pub fn is_target_file(file_path: &Path, patterns: &[Pattern]) -> bool {
  patterns
    .iter()
    .any(|pattern| pattern.matches_path(file_path))
}

pub fn is_exclude_dir(entry: &DirEntry, exclude_patterns: &[Pattern]) -> bool {
  let path = entry.path();
  exclude_patterns.iter().any(|p| p.matches_path(path))
}

pub fn find_local_components(root_path: &str, dirs: Vec<ConfigRegex>) -> HashSet<ComponentInfo> {
  let mut all_components = HashSet::new();
  let exclude_patterns = vec![Pattern::new("**/node_modules/**").expect("Invalid pattern")];
  let exclude = vec![];
  let filter = PathFilter::new(&dirs, &exclude);
  let patterns = [
    Pattern::new("**/*.tsx").unwrap(),
    Pattern::new("**/*.jsx").unwrap(),
  ];

  let walker = WalkDir::new(root_path).into_iter();
  let filtered_entries = walker
    .filter_entry(move |e| !is_exclude_dir(e, &exclude_patterns))
    .filter_map(Result::ok)
    .filter(|e| {
      e.file_type().is_file()
        && filter.execute(e.path().to_str().unwrap())
        && is_target_file(e.path(), &patterns)
    });

  for entry in filtered_entries {
    let file_path = entry.path().to_path_buf();
    all_components.extend(gen_components_by_file(&file_path));
  }

  all_components
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;
  #[test]
  fn test_find_local_components() {
    let current_dir = env::current_dir().unwrap();
    let binding = current_dir.join("playground");
    let root_path = binding.to_str().unwrap();
    let components = find_local_components(root_path, vec![]);
    assert!(!components.is_empty(), "Components should not be empty");
  }
}
