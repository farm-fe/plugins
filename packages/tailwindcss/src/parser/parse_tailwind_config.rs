use crate::config::{TailwindCssConfig, TailwindRsConfig};
use css_color::Srgb;
use std::{collections::BTreeMap, str::FromStr};
use tailwind_css::{
  BreakPointSystem, FontSize, FontSystem, Palette, PaletteSystem, PreflightSystem,
};
pub fn parse_tailwind_config(config: &TailwindCssConfig) -> TailwindRsConfig {
  let mut rs_config = TailwindRsConfig {
    palettes: None,
    fonts: None,
    preflight: None,
    screens: None,
  };

  if let Some(theme) = config.theme.as_ref() {
    // if let Some(colors) = theme.colors.as_ref() {
    //   let mut palettes = PaletteSystem::default();
    //   for (key, value) in colors.iter() {
    //     let mut colors = BTreeMap::default();
    //     colors.insert(50, Srgb::from_str(value).unwrap());
    //     // TODO Palette is private
    //     palettes.register(
    //       key.to_string(),
    //       Palette {
    //         gradient: false,
    //         key_points: colors,
    //       },
    //     );
    //   }
    //   rs_config.palettes = Some(palettes);
    // }
    if let Some(fonts) = theme.fonts.as_ref() {
      let size = fonts.size.clone();
      let family = fonts.family.clone();
      let tracking = fonts.tracking.clone();
      let mut rs_fonts = FontSystem::default();
      size.into_iter().for_each(|s| {
        rs_fonts.insert_size(s.0, FontSize::new(s.1.size, s.1.height));
      });
      family.into_iter().for_each(|f| {
        rs_fonts.insert_family(f.0, &f.1);
      });
      tracking.into_iter().for_each(|t| {
        rs_fonts.insert_tracking(t.0, t.1);
      });

      rs_config.fonts = Some(rs_fonts);
    }

    if let Some(preflight) = theme.preflight.as_ref() {
      let mut rs_preflight = PreflightSystem::default();
      rs_preflight.disable = preflight.disable;
      rs_preflight.remove_margins = preflight.remove_margins;
      rs_preflight.unstyle_head = preflight.unstyle_head;
      rs_preflight.unstyle_list = preflight.unstyle_list;
      rs_preflight.block_level_image = preflight.block_level_image;
      rs_preflight.unstyle_border = preflight.unstyle_border;
      rs_preflight.button_outline = preflight.button_outline;
      rs_preflight.custom = preflight.custom.clone();
      rs_config.preflight = Some(rs_preflight);
    }

    if let Some(screens) = theme.screens.as_ref() {
      let mut rs_screes = BreakPointSystem::default();
      screens.into_iter().for_each(|s| {
        rs_screes.register(s.0.clone(), s.1.clone());
      })
    }
  }
  rs_config
}
