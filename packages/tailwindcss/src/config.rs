use serde::{self};
use std::collections::BTreeMap;
use tailwind_css::{
  BreakPointSystem, FontSystem as RsFontSystem, PaletteSystem, PreflightSystem as RsPreflightSystem,
};

use std::collections::HashMap;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct FontSize {
  pub size: f32,
  pub height: f32,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct FontSystem {
  pub size: BTreeMap<String, FontSize>,
  pub family: BTreeMap<String, String>,
  pub tracking: BTreeMap<String, f32>,
}

// BreakPointSystem
#[derive(serde::Deserialize, Clone, Debug)]
pub struct BreakPoint {
  /// min-width
  /// unit: px
  pub width: usize,
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq)]
pub struct Srgb {
  /// The red component.
  pub red: f32,
  /// The green component.
  pub green: f32,
  /// The blue component.
  pub blue: f32,
  /// The alpha component.
  pub alpha: f32,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Palette {
  /// Allow gradients?
  pub gradient: bool,
  /// min-width
  /// unit: px
  pub key_points: BTreeMap<u32, Srgb>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct PreflightSystem {
  /// disable all preflight
  pub disable: bool,
  /// ## Default margins are removed
  /// Preflight removes all of the default margins from elements like headings, blockquotes, paragraphs, etc.
  /// This makes it harder to accidentally rely on margin values applied by the user-agent stylesheet that are not part of your spacing scale.
  pub remove_margins: bool,
  /// ## Headings are unstyled
  /// All heading elements are completely unstyled by default, and have the same font-size and font-weight as normal text.
  pub unstyle_head: bool,
  /// ## Lists are unstyled
  /// Ordered and unordered lists are unstyled by default, with no bullets/numbers and no margin or padding.
  pub unstyle_list: bool,
  /// ## Images are block-level
  /// Images and other replaced elements (like svg, video, canvas, and others) are display: block by default.
  pub block_level_image: bool,
  /// ## Border styles are reset globally
  /// In order to make it easy to add a border by simply adding the border class, Tailwind overrides the default border styles for all elements with the following rules:
  pub unstyle_border: bool,
  /// ## Buttons have a default outline
  /// To ensure that we provide accessible styles out of the box, we made sure that buttons have a default outline. You can of course override this by applying focus:ring or similar utilities to your buttons.
  pub button_outline: bool,
  /// Custom field for preflight
  pub custom: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
  pub colors: Option<HashMap<String, String>>,
  pub fonts: Option<FontSystem>,
  pub preflight: Option<PreflightSystem>,
  pub screens: Option<BTreeMap<String, usize>>,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TailwindCssConfig {
  pub content: Option<Vec<String>>,
  pub theme: Option<Theme>,
  // compile other config eg: inline  trace ... ?
}

pub struct TailwindRsConfig {
  pub palettes: Option<PaletteSystem>,
  pub fonts: Option<RsFontSystem>,
  pub preflight: Option<RsPreflightSystem>,
  pub screens: Option<BreakPointSystem>,
}

// tailwindcss_oxide config

pub struct TailwindOxideConfig {
  pub content: Option<Vec<String>>,
}
