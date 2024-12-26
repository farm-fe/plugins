use farmfe_core::{
  error::Result as HookResult,
  plugin::{PluginTransformHookParam, PluginTransformHookResult},
};
use regex::Regex;

pub fn transform(
  options: &crate::options::Options,
  param: &PluginTransformHookParam,
) -> HookResult<Option<PluginTransformHookResult>> {
  let content = param.content.clone();
  if let Some(ref name) = options.library_name {
    let import_regex_pattern = format!(
      r#"import\s*\{{\s*(\w+)\s*\}}\s*from\s*['"]{}['"]\s*;?"#,
      regex::escape(&name)
    );
    let import_regex = Regex::new(&import_regex_pattern).expect("Failed to create regex");

    if import_regex.is_match(&content) {
      let modified_content = import_regex.replace_all(&content, |caps: &regex::Captures| {
        let component_name = &caps[1];
        let formatted_component_name = if options.camel2_dash.unwrap_or(false) {
          format!(
            "{}{}",
            &component_name[..1].to_uppercase(),
            &component_name[1..]
          )
        } else {
          format!(
            "{}{}",
            &component_name[..1].to_lowercase(),
            &component_name[1..]
          )
        };

        let style_path = if let Some(ref style_library_name) = options.style_library_name {
          format!(
            "{}/{}/{}/{}/{}",
            name,
            options.style_lib_dir.clone().unwrap_or_default(),
            style_library_name,
            formatted_component_name,
            options
              .style_library_path
              .clone()
              .unwrap_or_default()
              .replace("[module]/", "")
          )
        } else {
          format!(
            "{}/{}/{}/{}",
            name,
            options.style_lib_dir.clone().unwrap_or_default(),
            formatted_component_name,
            options
              .style_library_path
              .clone()
              .unwrap_or_default()
              .replace("[module]/", "")
          )
        };
        format!(
          "import {} from '{}/{}/{}';\nimport '{}';\n",
          component_name,
          name,
          options.lib_dir.clone().unwrap_or_default(),
          formatted_component_name,
          style_path
        )
      });
      println!("Modified content with replacement: {}", modified_content);
      return Ok(Some(PluginTransformHookResult {
        content: modified_content.to_string(),
        module_type: None,
        source_map: None,
        ignore_previous_source_map: true,
      }));
    } else {
      println!("Content matches import pattern: false");
    }
  }
  Ok(None)
}
