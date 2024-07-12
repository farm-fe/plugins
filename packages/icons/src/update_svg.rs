use serde::Deserialize;
use xmltree::Element;

#[derive(Deserialize, Default, Clone)]
pub struct SvgModifier {
  pub fill: Option<String>,
  pub stroke: Option<String>,
  pub stroke_width: Option<String>,
  pub width: Option<String>,
  pub height: Option<String>,
  pub class: Option<String>,
  pub style: Option<serde_json::Value>,
  pub view_box: Option<String>,
}

impl SvgModifier {
  pub fn new(parmas: SvgModifier) -> Self {
    Self { ..parmas }
  }
  pub fn apply_to_svg(&self, svg_content: &str) -> String {
    let mut svg_element = Element::parse(svg_content.as_bytes()).unwrap();

    if let Some(ref fill) = self.fill {
      svg_element
        .attributes
        .insert("fill".to_string(), fill.clone());
    }
    if let Some(ref stroke) = self.stroke {
      svg_element
        .attributes
        .insert("stroke".to_string(), stroke.clone());
    }
    if let Some(ref stroke_width) = self.stroke_width {
      svg_element
        .attributes
        .insert("stroke-width".to_string(), stroke_width.clone());
    }
    if let Some(ref width) = self.width {
      svg_element
        .attributes
        .insert("width".to_string(), width.clone());
    }
    if let Some(ref height) = self.height {
      svg_element
        .attributes
        .insert("height".to_string(), height.clone());
    }
    if let Some(ref view_box) = self.view_box {
      svg_element
        .attributes
        .insert("viewBox".to_string(), view_box.clone());
    }
    if let Some(ref class) = self.class {
      svg_element
        .attributes
        .insert("class".to_string(), class.clone());
    }
    if let Some(ref style) = self.style {
      let style_str = style.as_object().map_or(String::new(), |obj| {
        obj
          .iter()
          .map(|(key, value)| {
            format!(
              "{}:{};",
              key,
              value.as_str().unwrap_or("").replace("\"", "")
            )
          })
          .collect::<Vec<_>>()
          .join(" ")
      });
      svg_element
        .attributes
        .insert("style".to_string(), style_str);
    }

    let mut new_svg_content = Vec::new();
    svg_element.write(&mut new_svg_content).unwrap();
    String::from_utf8(new_svg_content).unwrap()
  }
}
