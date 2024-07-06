use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GenSvgElement {
  pub fill: Option<String>,
  pub stroke: Option<String>,
  pub stroke_width: Option<f32>,
  pub width: Option<u32>,
  pub height: Option<u32>,
  pub path: Option<String>,
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
        .map(|v| format!(r#" stroke-width="{}""#, v)),
      self.width.map(|v| format!(r#" width="{}""#, v)),
      self.height.map(|v| format!(r#" height="{}""#, v)),
    ];

    for attr in attributes.iter().flatten() {
      svg.push_str(attr);
    }

    if let Some(path) = &self.path {
      svg.push_str(&format!(r#">{}</path>"#, path));
    }
    svg.push_str("</svg>");
    svg
  }
}
