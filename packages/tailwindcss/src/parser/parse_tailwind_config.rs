use crate::config::{TailwindCssConfig, TailwindRsConfig};
use css_color::Srgb;
use std::{collections::BTreeMap, str::FromStr};
use tailwind_css::{BreakPointSystem, Palette, PaletteSystem};
pub fn parse_tailwind_config(
  config: &TailwindCssConfig,
) -> Result<TailwindRsConfig, serde_json::Error> {
  let mut rs_config = TailwindRsConfig {
    palettes: None,
    fonts: None,
    preflight: None,
    screens: None,
  };

  if let Some(theme) = config.theme.as_ref() {
    if let Some(colors) = theme.colors.as_ref() {
      let mut palettes = PaletteSystem::default();
      for (key, value) in colors.iter() {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str(value).unwrap());
        // TODO Palette is private
        palettes.register(
          key.to_string(),
          Palette {
            gradient: false,
            key_points: colors,
          },
        );
      }
      rs_config.palettes = Some(palettes);
    }
    if let Some(_fonts) = theme.fonts.as_ref() {
      rs_config.fonts = theme.fonts.clone();
    }

    if let Some(_) = theme.preflight.as_ref() {
      rs_config.preflight = theme.preflight.clone()
    }

    if let Some(screens) = theme.screens.as_ref() {
      let mut rs_screes = BreakPointSystem::default();
      screens.into_iter().for_each(|s| {
        rs_screes.register(s.0.clone(), s.1.clone());
      })
    }
  }
  Ok(rs_config)
}
