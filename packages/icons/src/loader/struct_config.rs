use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IconifyJSON {
  pub icons: HashMap<String, IconifyIcon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IconifyIcon {
  pub left: Option<i32>,
  pub top: Option<i32>,
  pub width: Option<i32>,
  pub height: Option<i32>,
  pub body: String,
  pub h_flip: Option<bool>,
  pub v_flip: Option<bool>,
  pub rotate: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct IconifyLoaderOptions {
  pub customizations: Option<IconifyIconCustomisations>,
  pub scale: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct IconifyIconCustomisations {
  pub width: Option<String>,
  pub height: Option<String>,
}
