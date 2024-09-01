use crate::config::TailwindCssConfig;
use crate::parser::parse_tailwind_config::parse_tailwind_config;
use tailwind_css::TailwindBuilder;

fn get_tailwind_builder(config: &TailwindCssConfig) -> TailwindBuilder {
  let tw_config = parse_tailwind_config(config);
  let mut builder = TailwindBuilder::default();
  if tw_config.fonts.is_some() {
    builder.fonts = tw_config.fonts.unwrap();
  }
  if tw_config.palettes.is_some() {
    builder.palettes = tw_config.palettes.unwrap();
  }
  if tw_config.preflight.is_some() {
    builder.preflight = tw_config.preflight.unwrap();
  }
  if tw_config.screens.is_some() {
    builder.screens = tw_config.screens.unwrap();
  }
  builder
}
