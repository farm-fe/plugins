#![deny(clippy::all)]
mod config;
mod parser;

use std::{
  path::Path,
  sync::{Arc, Mutex},
};

use config::TailwindCssConfig;
use farmfe_core::{
  config::Config,
  module::ModuleType,
  plugin::{
    Plugin, PluginAnalyzeDepsHookResultEntry, PluginLoadHookResult,
    PluginRenderResourcePotHookResult, ResolveKind,
  },
  resource::resource_pot::ResourcePotType,
  serde_json::{self},
};
use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::hash::sha256;
use parser::{
  get_tailwind_builder::get_tailwind_builder,
  get_tailwind_scanner::get_tailwind_scanner,
  parse_tailwind_css::{collect_tailwind_css, collect_tailwind_css_with_changed},
};
use tailwind_css::TailwindBuilder;
use tailwindcss_oxide::Scanner;

const GLOBAL_INJECT_MODULE_ID: &str = "farmfe_plugin_tailwindcss_global_inject";

#[farm_plugin]
pub struct FarmfePluginTailwindcss {
  tw_config: TailwindCssConfig,
  tw_builder: Arc<Mutex<TailwindBuilder>>,
  tw_scanner: Arc<Mutex<Scanner>>,
}
impl FarmfePluginTailwindcss {
  fn new(config: &Config, options: String) -> Self {
    let base = config.root.clone();
    let tw_config: TailwindCssConfig = serde_json::from_str(&options).unwrap();
    let contents = tw_config.content.clone();
    if contents.is_none() {
      panic!("tailwindcss config content is required");
    }
    let mut tw_scanner = get_tailwind_scanner(&base, contents.unwrap());
    let mut tw_builder = get_tailwind_builder(&tw_config);
    collect_tailwind_css(&mut tw_builder, &mut tw_scanner);
    Self {
      tw_config,
      tw_builder: Arc::new(Mutex::new(tw_builder)),
      tw_scanner: Arc::new(Mutex::new(tw_scanner)),
    }
  }
}

impl Plugin for FarmfePluginTailwindcss {
  fn name(&self) -> &str {
    "FarmfePluginTailwindcss"
  }
  fn priority(&self) -> i32 {
    100
  }
  fn resolve(
    &self,
    param: &farmfe_core::plugin::PluginResolveHookParam,
    _context: &Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginResolveHookResult>> {
    if param.source == GLOBAL_INJECT_MODULE_ID {
      return Ok(Some(farmfe_core::plugin::PluginResolveHookResult {
        resolved_path: GLOBAL_INJECT_MODULE_ID.to_string(),
        ..Default::default()
      }));
    }
    return Ok(None);
  }
  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    if param.resolved_path == GLOBAL_INJECT_MODULE_ID {
      return Ok(Some(PluginLoadHookResult {
        content: format!(
          ".farmfe_plugin_tailwindcss_global_inject{}{}",
          sha256(GLOBAL_INJECT_MODULE_ID.as_bytes(), 8).to_string(),
          "{}"
        ),
        module_type: ModuleType::Css,
        source_map: None,
      }));
    }
    return Ok(None);
  }
  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    _context: &Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    if !param.resolved_path.contains("node_modules")
      && Path::new(param.resolved_path).is_file()
      && vec![
        ModuleType::Tsx,
        ModuleType::Jsx,
        ModuleType::Js,
        ModuleType::Html,
      ]
      .contains(&param.module_type)
    {
      let mut tw_scanner = self.tw_scanner.lock().unwrap();
      let mut tw_builder = self.tw_builder.lock().unwrap();
      let changed_files = vec![param.resolved_path.to_string()];
      let _is_tailwind =
        collect_tailwind_css_with_changed(&mut tw_builder, &mut tw_scanner, changed_files);
    }
    return Ok(None);
  }

  // fn build_end(&self, context: &Arc<CompilationContext>) -> farmfe_core::error::Result<Option<()>> {
  //   if !matches!(context.config.mode, farmfe_core::config::Mode::Development)
  //     || !matches!(context.config.output.target_env, TargetEnv::Browser)
  //   {
  //     return Ok(None);
  //   }

  //   // transform all css to script
  //   let css_modules = context
  //     .module_graph
  //     .write()
  //     .modules()
  //     .into_iter()
  //     .filter_map(|m| {
  //       if matches!(m.module_type, ModuleType::Css) {
  //         Some(m.id.clone())
  //       } else {
  //         None
  //       }
  //     })
  //     .collect::<Vec<ModuleId>>();
  //   let tw_code = self.tw_builder.lock().unwrap().bundle().unwrap();
  //   transform_css_to_script::transform_css_to_script_modules(
  //     GLOBAL_INJECT_MODULE_ID,
  //     tw_code,
  //     css_modules,
  //     context,
  //   )?;

  //   Ok(Some(()))
  // }

  // fn module_graph_updated(
  //   &self,
  //   param: &farmfe_core::plugin::PluginModuleGraphUpdatedHookParams,
  //   context: &Arc<CompilationContext>,
  // ) -> farmfe_core::error::Result<Option<()>> {
  //   let mut module_ids = param.updated_modules_ids.clone();
  //   module_ids.extend(param.added_modules_ids.clone());
  //   let tw_code = self.tw_builder.lock().unwrap().bundle().unwrap();
  //   transform_css_to_script::transform_css_to_script_modules(
  //     GLOBAL_INJECT_MODULE_ID,
  //     tw_code,
  //     module_ids,
  //     context,
  //   )?;

  //   Ok(Some(()))
  // }

  fn render_resource_pot(
    &self,
    param: &farmfe_core::plugin::PluginRenderResourcePotHookParam,
    _context: &Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginRenderResourcePotHookResult>>
  {
    if param.resource_pot_info.resource_pot_type == ResourcePotType::Css
      || param.resource_pot_info.resource_pot_type == ResourcePotType::Js
    {
      if param
        .resource_pot_info
        .module_ids
        .iter()
        .any(|m| m.to_string() == GLOBAL_INJECT_MODULE_ID)
      {
        let tw_builder = self.tw_builder.lock().unwrap();
        let code = tw_builder.bundle().unwrap();
        return Ok(Some(PluginRenderResourcePotHookResult {
          content: param.content.replace(
            &format!(
              ".farmfe_plugin_tailwindcss_global_inject{} {}",
              sha256(GLOBAL_INJECT_MODULE_ID.as_bytes(), 8).to_string(),
              "{}"
            ),
            &code,
          ),
          source_map: None,
        }));
      }
    }
    Ok(None)
  }

  fn analyze_deps(
    &self,
    param: &mut farmfe_core::plugin::PluginAnalyzeDepsHookParam,
    _context: &Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<()>> {
    if param.module.module_type == farmfe_core::module::ModuleType::Html {
      param.deps.insert(
        0,
        PluginAnalyzeDepsHookResultEntry {
          source: GLOBAL_INJECT_MODULE_ID.to_string(),
          kind: ResolveKind::CssUrl,
        },
      );
    }
    Ok(Some(()))
  }
}
