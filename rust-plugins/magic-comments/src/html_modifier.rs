use lol_html::{element, html_content::ContentType, HtmlRewriter, Settings};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Attrs {
  pub rel: String,
  pub crossorigin: Option<bool>,
  pub href: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Tag {
  pub tag: String,
  pub inject_to: String,
  pub attrs: Attrs,
}

pub fn inject_tags(html: &str, tags: Vec<Tag>) -> Result<String, Box<dyn Error>> {
  let mut output = Vec::new();
  let tags = &tags;

  let mut rewriter = HtmlRewriter::new(
    Settings {
      element_content_handlers: vec![element!("head, body", move |el| {
        let binding = el.tag_name();
        let element_name = binding.as_str();
        for tag in tags {
          if tag.inject_to == element_name {
            let mut attrs_html = format!("rel=\"{}\" ", tag.attrs.rel);

            if let Some(true) = tag.attrs.crossorigin {
              attrs_html.push_str("crossorigin ");
            }

            attrs_html.push_str(&format!("href=\"{}\"", tag.attrs.href));

            el.append(&format!("<{} {}>", tag.tag, attrs_html), ContentType::Html);
          }
        }
        Ok(())
      })],
      ..Settings::default()
    },
    |c: &[u8]| output.extend_from_slice(c),
  );

  rewriter.write(html.as_bytes())?;
  rewriter.end()?;

  Ok(String::from_utf8(output)?)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_inject_tags() {
    let html = r#"<!DOCTYPE html>
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <h1>Hello World</h1>
            </body>
        </html>"#;

    let tags = vec![Tag {
      tag: "link".to_string(),
      inject_to: "head".to_string(),
      attrs: Attrs {
        rel: "modulepreload".to_string(),
        crossorigin: Some(true),
        href: "/assets/main.js".to_string(),
      },
    }];

    let result = inject_tags(html, tags).unwrap();
    println!("result {:?}", result);
    assert!(result.contains("modulepreload"));
    assert!(result.contains("crossorigin"));
    assert!(result.contains("/assets/main.js"));
  }
}
