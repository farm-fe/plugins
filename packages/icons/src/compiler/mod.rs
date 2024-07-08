use farmfe_core::module::ModuleType;

pub mod preact;
pub mod react;
pub mod solid;
pub mod svelte;
pub mod vue;

pub struct GetCompilerParams {
  pub jsx: String,
  pub compiler: String,
}

pub struct CompilerParams {
  pub svg: String,
  pub svg_name: String,
  pub root_path: String,
}

pub fn get_compiler(param: GetCompilerParams) -> impl Fn(CompilerParams) -> String {
  let GetCompilerParams { compiler, jsx } = param;
  match compiler.as_str() {
    "jsx" => {
      if jsx == "react" {
        react::react_complier
      } else {
        preact::preact_complier
      }
    }
    "svelte" => svelte::svelte_compiler,
    "vue" => vue::vue_compiler,
    "solid" => solid::solid_compiler,
    _ => panic!("Unsupported extension: {}", compiler),
  }
}

pub fn get_module_type_by_path(param: GetCompilerParams) -> ModuleType {
  let GetCompilerParams { compiler, .. } = param;
  match compiler.as_str() {
    "jsx" => ModuleType::Jsx,
    "svelte" => ModuleType::Js,
    "vue" => ModuleType::Js,
    "solid" => ModuleType::Js,
    _ => panic!("Unsupported extension: {}", compiler),
  }
}
