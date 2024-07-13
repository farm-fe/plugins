use farmfe_core::regex::Regex;

pub struct Options {
  /// - Explicitly set the Vue version
  /// - default 3
  vue_version: Option<String>,
  head_enabled: Option<bool>,
  head_field: Option<String>,
  frontmatter: Option<bool>,
  excerpt: Option<bool>,
  custom_sfc_blocks: Option<Vec<String>>,
  // todo component_options frontmatterOptions frontmatterPreprocess
  // todo markdownItOptions markdownItUses markdownItSetup wrapperClasses wrapperComponent
  // todo transforms
  expose_frontmatter: Option<bool>,
  export_frontmatter: Option<bool>,
  escape_code_tag_interpolation: Option<bool>,
  include: Option<Regex>,
  exclude: Option<Regex>,
}
