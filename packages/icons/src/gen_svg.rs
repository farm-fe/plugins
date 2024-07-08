use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct GenSvgElement {
  pub path: Option<String>,
  pub fill: Option<String>,
  pub stroke: Option<String>,
  pub stroke_width: Option<String>,
  pub width: Option<String>,
  pub height: Option<String>,
  pub class: Option<String>,
  pub style: Option<Value>,
  pub scale: Option<f32>,
}

impl GenSvgElement {
  pub fn new(config: GenSvgElement) -> Self {
    GenSvgElement { ..config }
  }

  pub fn apply_to_svg(&self) -> String {
    let mut svg = String::from("<svg");

    let attributes = [
      self.fill.as_ref().map(|v| format!(r#" fill="{}""#, v)),
      self.stroke.as_ref().map(|v| format!(r#" stroke="{}""#, v)),
      self
        .stroke_width
        .as_ref()
        .map(|v| format!(r#" stroke-width="{}""#, v)),
      self.width.as_ref().map(|v| format!(r#" width="{}""#, v)),
      self.height.as_ref().map(|v| format!(r#" height="{}""#, v)),
      self.class.as_ref().map(|v| format!(r#" class="{}""#, v)),
      self.style.as_ref().map(|v| {
        let style_str = v.as_object().map_or(String::new(), |obj| {
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
            .join("")
        });
        format!(r#" style="{}""#, style_str)
      }),
      self.scale.map(|v| format!(r#" transform="scale({})""#, v)),
    ];

    for attr in attributes.iter().flatten() {
      svg.push_str(attr);
    }

    svg.push('>');
    if let Some(path) = &self.path {
      svg.push_str(path);
    }
    svg.push_str("</svg>");
    svg
  }
}
