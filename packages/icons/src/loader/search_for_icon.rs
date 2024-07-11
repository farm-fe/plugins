use super::super::update_svg::SvgModifier;
use super::{
  icon_to_svg::{icon_to_svg, IconifyIconBuildResult},
  struct_config::{IconifyIcon, IconifyLoaderOptions},
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
    } = icon_to_svg(icon, None);
    let scale = options.as_ref().and_then(|opts| opts.scale);
    if let Some(s) = scale {
      if attributes.height.is_none() {
        attributes.height = Some(format!("{}{}", s, "em"));
      }
      if attributes.width.is_none() {
        attributes.width = Some(format!("{}{}", s, "em"));
      }
    }
    let svg = SvgModifier::new(SvgModifier {
      fill: None,
      stroke: None,
      stroke_width: None,
      width: attributes.width,
      height: attributes.height,
      class: None,
      style: None,
      view_box: Some(attributes.view_box),
    });
    return Some(svg.apply_to_svg(&format!("<svg>{}</svg>", body)));
  } else {
    None
  }
}

// fn get_icon_data(icon_set: &IconifyJSON, id: &str) -> Option<IconifyIcon> {
//   // 实现获取图标数据的逻辑
//   None
// }

// fn merge_icon_props(svg: &str, collection: &str, scale: Option<f64>) -> String {
//   // 实现合并图标属性的逻辑
//   svg.to_string()
// }

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
