use super::calculate_size::calculate_size;
use super::struct_config::IconifyIcon;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IconifyIconCustomizations {
  pub width: Option<String>,
  pub height: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IconifyIconBuildResult {
  pub attributes: Attributes,
  pub view_box: SVGViewBox,
  pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
  pub width: Option<String>,
  pub height: Option<String>,
  pub view_box: String,
}

type SVGViewBox = (i32, i32, i32, i32);

fn default_icon_customisations() -> IconifyIconCustomizations {
  IconifyIconCustomizations {
    width: None,
    height: None,
  }
}

fn wrap_svg_content(body: &str, prefix: &str, suffix: &str) -> String {
  format!("{}{}{}", prefix, body, suffix)
}

pub fn icon_to_svg(
  icon: IconifyIcon,
  customizations: Option<IconifyIconCustomizations>,
) -> IconifyIconBuildResult {
  let full_customizations = customizations.unwrap_or_else(default_icon_customisations);

  let mut box_left = icon.left.unwrap_or(0);
  let mut box_top = icon.top.unwrap_or(0);
  let mut box_width = icon.width.unwrap_or(16);
  let mut box_height = icon.height.unwrap_or(16);

  let mut body = icon.body.clone();

  let mut transformations: Vec<String> = Vec::new();
  let mut rotation = icon.rotate.unwrap_or(0);

  if icon.h_flip.unwrap_or(false) {
    if icon.v_flip.unwrap_or(false) {
      rotation += 2;
    } else {
      transformations.push(format!("translate({} {})", box_width + box_left, -box_top));
      transformations.push("scale(-1 1)".to_string());
      box_top = 0;
      box_left = 0;
    }
  } else if icon.v_flip.unwrap_or(false) {
    transformations.push(format!("translate({} {})", -box_left, box_height + box_top));
    transformations.push("scale(1 -1)".to_string());
    box_top = 0;
    box_left = 0;
  }

  if rotation < 0 {
    rotation -= (rotation / 4) * 4;
  }
  rotation %= 4;

  match rotation {
    1 => {
      let temp_value = box_height / 2 + box_top;
      transformations.insert(0, format!("rotate(90 {} {})", temp_value, temp_value));
    }
    2 => {
      transformations.insert(
        0,
        format!(
          "rotate(180 {} {})",
          box_width / 2 + box_left,
          box_height / 2 + box_top
        ),
      );
    }
    3 => {
      let temp_value = box_width / 2 + box_left;
      transformations.insert(0, format!("rotate(-90 {} {})", temp_value, temp_value));
    }
    _ => {}
  }

  if rotation % 2 == 1 {
    if box_left != box_top {
      let temp_value = box_left;
      box_left = box_top;
      box_top = temp_value;
    }
    if box_width != box_height {
      let temp_value = box_width;
      box_width = box_height;
      box_height = temp_value;
    }
  }

  if !transformations.is_empty() {
    body = wrap_svg_content(
      &body,
      &format!("<g transform=\"{}\">", transformations.join(" ")),
      "</g>",
    );
  }

  let customizations_width = full_customizations.width.clone();
  let customizations_height = full_customizations.height.clone();

  let mut width: Option<String> = None;
  let mut height: Option<String> = None;

  if customizations_width.is_none() {
    height = if customizations_height.is_none() {
      Some("1em".to_string())
    } else if customizations_height.as_deref() == Some("auto") {
      Some(box_height.to_string())
    } else {
      Some(customizations_height.unwrap())
    };
    width = if height.is_none() {
      None
    } else {
      Some(calculate_size(
        &height.clone().unwrap(),
        box_width as f32 / box_height as f32,
        None,
      ))
    }
  } else {
    width = if customizations_width.as_deref() == Some("auto") {
      Some(box_width.to_string())
    } else {
      Some(customizations_width.unwrap())
    };
    height = if customizations_height.is_none() {
      Some(calculate_size(
        &width.clone().unwrap(),
        box_height as f32 / box_width as f32,
        None,
      ))
    } else if customizations_height.as_deref() == Some("auto") {
      Some(box_height.to_string())
    } else {
      Some(customizations_height.unwrap())
    };
  }

  let attributes = Attributes {
    width,
    height,
    view_box: format!("{} {} {} {}", box_left, box_top, box_width, box_height),
  };

  let view_box = (box_left, box_top, box_width, box_height);

  IconifyIconBuildResult {
    attributes,
    view_box,
    body,
  }
}
