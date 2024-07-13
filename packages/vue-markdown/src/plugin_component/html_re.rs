use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
  pub static ref ATTR_NAME: Regex = Regex::new(r"[a-zA-Z_:@][a-zA-Z0-9:._-]*").unwrap();
  pub static ref SINGLE_QUOTED: Regex = Regex::new(r"'[^']*'").unwrap();
  pub static ref DOUBLE_QUOTED: Regex = Regex::new(r#""[^"]*""#).unwrap();
  pub static ref ATTR_VALUE: Regex = Regex::new(&format!(
    "(?:{}|{}|{})",
    *UNQUOTED, *SINGLE_QUOTED, *DOUBLE_QUOTED
  ))
  .unwrap();
  pub static ref ATTRIBUTE: Regex = Regex::new(&format!(
    r"(?:\s+{}(?:\s*=\s*{})?)",
    *ATTR_NAME, *ATTR_VALUE
  ))
  .unwrap();
  pub static ref OPEN_TAG: Regex =
    Regex::new(&format!("<[A-Za-z][A-Za-z0-9\\-]*{}*\\s*/?>", *ATTRIBUTE)).unwrap();
  pub static ref CLOSE_TAG: Regex = Regex::new(r"</[A-Za-z][A-Za-z0-9\-]*\s*>").unwrap();
  pub static ref COMMENT: Regex = Regex::new(r"<!---->|<!--(?:-?[^>-])(?:-?[^-])*-->").unwrap();
  pub static ref PROCESSING: Regex = Regex::new(r"<[?][\s\S]*?[?]>").unwrap();
  pub static ref DECLARATION: Regex = Regex::new(r"<![A-Z]+\s+[^>]*>").unwrap();
  pub static ref CDATA: Regex = Regex::new(r"<!\[CDATA\[[\s\S]*?\]\]>").unwrap();
  pub static ref HTML_TAG_RE: Regex = Regex::new(&format!(
    "^(?:{}|{}|{}|{}|{}|{})",
    *OPEN_TAG, *CLOSE_TAG, *COMMENT, *PROCESSING, *DECLARATION, *CDATA
  ))
  .unwrap();
  pub static ref HTML_OPEN_CLOSE_TAG_RE: Regex =
    Regex::new(&format!("^(?:{}|{})", *OPEN_TAG, *CLOSE_TAG)).unwrap();
  pub static ref HTML_SELF_CLOSING_TAG_RE: Regex =
    Regex::new(&format!("^<[A-Za-z][A-Za-z0-9\\-]*{}*\\s*/>", *ATTRIBUTE)).unwrap();
  pub static ref HTML_OPEN_AND_CLOSE_TAG_IN_THE_SAME_LINE_RE: Regex = Regex::new(&format!(
    "^<([A-Za-z][A-Za-z0-9\\-]*){}*\\s*>.*</\\1\\s*>",
    *ATTRIBUTE
  ))
  .unwrap();
  pub static ref unquoted: Regex = Regex::new(r#"[^"\'=<>`\x00-\x20]+"#).unwrap();
}
