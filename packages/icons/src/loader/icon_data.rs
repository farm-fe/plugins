use super::svg_modifier::SvgModifier;
use super::struct_config::IconifyLoaderOptions;
use super::{
  icon_to_svg::{icon_to_svg, IconifyIconBuildResult},
  struct_config::IconifyIcon,
};

pub fn gen_svg_for_icon_data(
  icon_data: Option<IconifyIcon>,
  options: Option<IconifyLoaderOptions>,
) -> Option<String> {
  if let Some(icon) = icon_data {
    let IconifyIconBuildResult {
      mut attributes,
      body,
      ..
    } = icon_to_svg(icon.clone(), None);
    let scale = options.as_ref().and_then(|o| o.scale);
    if let Some(s) = scale {
      if s != 0.0 {
        attributes.height = Some(format!("{}{}", s, "em"));
        attributes.width = Some(format!("{}{}", s, "em"));
      }
    }
    let svg = SvgModifier::new(SvgModifier {
      width: attributes.width,
      height: attributes.height,
      view_box: Some(attributes.view_box),
      ..options
        .as_ref()
        .and_then(|o| o.customizations.clone())
        .unwrap_or_default()
    });
    return Some(svg.apply_to_svg(&format!("<svg>{}</svg>", body)));
  } else {
    panic!("Icon data is missing");
  }
}

