use farmfe_core::module::ModuleType;

pub mod preact;
pub mod react;
pub mod solid;
pub mod svelte;
pub mod vue;

pub struct GetCompilerParams {
  pub path: String,
  pub jsx: String,
}

pub struct CompilerParams {
  pub svg: String,
  pub root_path: String,
}

pub fn get_compiler(param: GetCompilerParams) -> impl Fn(CompilerParams) -> String {
  let GetCompilerParams { path, jsx } = param;
  let ext = path.split('.').last().unwrap();
  match ext {
    "jsx" => {
      if jsx == "react" {
        react::react_complier
      } else {
        preact::preact_complier
      }
    }
    "svelte" => svelte::svelte_compiler,
    "vue" => vue::vue_compiler,
    "tsx" => solid::solid_compiler,
    _ => panic!("Unsupported extension: {}", ext),
  }
}

pub fn get_module_type_by_path(param: GetCompilerParams) -> ModuleType {
  let GetCompilerParams { path, jsx } = param;
  let ext = path.split('.').last().unwrap();
  match ext {
    "jsx" => {
      if jsx == "react" {
        ModuleType::Jsx
      } else {
        ModuleType::Jsx
      }
    }
    "svelte" => ModuleType::Js,
    "vue" => ModuleType::Js,
    "tsx" => ModuleType::Js,
    _ => panic!("Unsupported extension: {}", ext),
  }
}
