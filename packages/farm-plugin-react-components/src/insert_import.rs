use farmfe_core::{swc_common::DUMMY_SP, swc_ecma_ast::*};
use farmfe_toolkit::swc_ecma_visit::{VisitMut, VisitMutWith};
use std::collections::HashSet;

use crate::find_local_components::{ComponentInfo, ExportType};
pub struct ImportModifier {
  pub components: HashSet<ComponentInfo>,
  pub used_components: HashSet<ComponentInfo>,
}

impl ImportModifier {
  pub fn new(components: HashSet<ComponentInfo>) -> Self {
    Self {
      components,
      used_components: HashSet::new(),
    }
  }
}

impl VisitMut for ImportModifier {
  fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
    for specifier in &n.specifiers {
      match specifier {
        ImportSpecifier::Default(default_spec) => {
          let imported_name = default_spec.local.sym.as_ref();
          self
            .components
            .retain(|c: &ComponentInfo| &c.name != imported_name);
        }

        ImportSpecifier::Named(named_spec) => {
          let imported_name = match &named_spec.imported {
            Some(imported) => match imported {
              ModuleExportName::Ident(ident) => ident.sym.as_ref(),
              ModuleExportName::Str(str) => str.value.as_ref(),
            },
            None => named_spec.local.sym.as_ref(),
          };
          self
            .components
            .retain(|c| &c.name != imported_name || c.name != named_spec.local.sym.as_ref());
        }
        _ => {}
      }
    }
  }
  fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
    if let JSXElementName::Ident(ident) = &n.name {
      let component_name = ident.sym.to_string();
      if component_name
        .chars()
        .next()
        .map_or(false, |c| c.is_uppercase())
      {
        let item = self.components.iter().find(|c| c.name == component_name);
        if let Some(item) = item {
          self.used_components.insert(item.clone());
        }
      }
    }

    n.visit_mut_children_with(self);
  }
}

pub struct InsertImportModifier {
  pub components: HashSet<ComponentInfo>,
}
impl InsertImportModifier {
  pub fn new(components: HashSet<ComponentInfo>) -> Self {
    Self { components }
  }
}
impl VisitMut for InsertImportModifier {
  fn visit_mut_module_items(&mut self, items: &mut Vec<ModuleItem>) {
    let mut last_import_index = None;
    for (index, item) in items.iter().enumerate() {
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(_)) = item {
        last_import_index = Some(index);
      }
    }

    let mut new_imports = Vec::new();
    for component in &self.components {
      let imported = {
        if component.original_name != component.name {
          Some(ModuleExportName::Ident(Ident::new(
            component.original_name.clone().into(),
            DUMMY_SP,
          )))
        } else {
          None
        }
      };
      let specifier = match component.export_type {
        ExportType::Default => ImportSpecifier::Default(ImportDefaultSpecifier {
          local: Ident::new(component.name.clone().into(), DUMMY_SP),
          span: DUMMY_SP,
        }),
        ExportType::Named => ImportSpecifier::Named(ImportNamedSpecifier {
          local: Ident::new(component.name.clone().into(), DUMMY_SP),
          imported,
          span: DUMMY_SP,
          is_type_only: false,
        }),
      };

      let import_decl = ImportDecl {
        src: Box::new(Str {
          value: component.path.clone().into(),
          span: DUMMY_SP,
          raw: None,
        }),
        specifiers: vec![specifier],
        type_only: false,
        span: DUMMY_SP,
        with: Default::default(),
        phase: Default::default(),
      };

      new_imports.push(ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)));
    }

    match last_import_index {
      Some(index) => items.splice(index + 1..index + 1, new_imports.iter().cloned()),
      None => items.splice(0..0, new_imports.iter().cloned()),
    };
  }
}
