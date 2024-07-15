use farmfe_core::regex::Regex;

pub struct Options {
  /// - Explicitly set the Vue version
  /// - default 3
  pub vue_version: Option<String>,
  pub head_enabled: Option<bool>,
  pub head_field: Option<String>,
  pub frontmatter: Option<bool>,
  pub excerpt: Option<bool>,
  pub custom_sfc_blocks: Option<Vec<String>>,
  // todo component_options frontmatterOptions frontmatterPreprocess
  // todo markdownItOptions markdownItUses markdownItSetup wrapperClasses wrapperComponent
  // todo transforms
  pub expose_frontmatter: Option<bool>,
  pub export_frontmatter: Option<bool>,
  pub escape_code_tag_interpolation: Option<bool>,
  pub include: Option<Regex>,
  pub exclude: Option<Regex>,
  pub wrapper_class: Option<String>,
  pub head_enabled: Option<bool>,

  

}
