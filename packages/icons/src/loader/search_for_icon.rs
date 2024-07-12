use super::super::update_svg::SvgModifier;
use super::struct_config::IconifyLoaderOptions;
use super::{
  icon_to_svg::{icon_to_svg, IconifyIconBuildResult},
  struct_config::IconifyIcon,
};
use serde::{Deserialize, Serialize};

pub fn search_for_icon(
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
      attributes.height = Some(format!("{}{}", s, "em"));
      attributes.width = Some(format!("{}{}", s, "em"));
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
    None
  }
}

#[derive(Serialize, Deserialize)]
struct Attributes {
  width: Option<f64>,
  height: Option<f64>,
  // 其他属性
}

// #[tokio::main]
// async fn main() {
//     // 初始化日志
//     env_logger::init();

//     // 示例调用search_for_icon函数
//     let icon_set = IconifyJSON {};
//     let collection = "example_collection";
//     let ids = vec!["icon1", "icon2"];
//     let options = Some(IconifyLoaderOptions {
//         customizations: None,
//         scale: Some(1.0),
//     });

//     if let Some(svg) = search_for_icon(icon_set, collection, ids, options).await {
//         println!("Found SVG: {}", svg);
//     } else {
//         println!("Icon not found");
//     }
// }
