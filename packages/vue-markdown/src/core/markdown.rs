extern crate markdown_it_front_matter;
use crate::core::options::Options;
use crate::core::utils::vue_compile;
use farmfe_core::serde_json;
use markdown_it::MarkdownIt;
use markdown_it_front_matter::FrontMatter;
use regex::Regex;
use serde_yaml::Value;

#[derive(Debug)]
struct ScriptMeta {
  code: String,
  attr: String,
}

fn extract_script_setup(html: &str) -> (String, Vec<ScriptMeta>) {
  let script_setup_re =
    Regex::new(r"<\sscript([^>]*?)\bsetup\b([^>]*?)>([\s\S]*?)</script>").unwrap();
  let mut scripts: Vec<_> = Vec::new();

  // 使用正则表达式替换匹配的部分，并收集脚本元数据
  let result_html = script_setup_re.replace_all(html, |caps: &regex::Captures| {
    scripts.push(ScriptMeta {
      code: caps[3].to_string(),
      attr: format!("{} {}", &caps[1], &caps[2]).trim().to_string(),
    });
    "" // 替换为空字符串
  });

  (result_html.to_string(), scripts) as (String, Vec<ScriptMeta>)
}

pub fn create_markdown(content: String, options: Options, file_name: String, id: String) -> String {
  // let script_setup_re =
  //   Regex::new(r"<\sscript([^>]*?)\bsetup\b([^>]*?)>([\s\S]*?)</script>").unwrap();

  let is_vue2 = if let Some(vue_version) = options.vue_version {
    vue_version.starts_with("2.")
  } else {
    false
  };

  let md = &mut MarkdownIt::new();
  markdown_it::plugins::cmark::add(md);
  markdown_it::plugins::html::add(md);
  markdown_it::plugins::extra::linkify::add(md);
  markdown_it::plugins::extra::typographer::add(md);
  markdown_it::plugins::extra::add(md);
  markdown_it::plugins::sourcepos::add(md);
  markdown_it_front_matter::add(md);

  let Options {
    wrapper_class,
    // head_enabled,
    ..
  } = options;

  let raw = content.trim_start();
  let node = md.parse(raw);
  let mut front_matter = String::from("--- \n ---");
  let front_matter_value = node
    .children
    .first()
    .unwrap()
    .node_value
    .as_any()
    .downcast_ref::<FrontMatter>();

  if !front_matter_value.is_none() {
    front_matter = front_matter_value.unwrap().content.clone();
  }

  let front_matter = serde_yaml::from_str::<Value>(&front_matter).unwrap();

  let front_matter = format!(
    "const frontmatter = {};",
    serde_json::to_string(&front_matter).unwrap()
  );

  let mut html = node.render();

  if wrapper_class.is_some() {
    html = format!("<div class=\"{}\">{}</div>", wrapper_class.unwrap(), html);
  } else {
    html = format!("<div>{}</div>", html);
  }

  let (new_html, new_scripts) = extract_script_setup(&html);
  html = new_html;

  let mut script_lines: Vec<_> = vec![];

  let hoist_scripts_lines: Vec<String> = new_scripts
    .iter()
    .map(|item| item.code.clone())
    .collect::<Vec<_>>();
  script_lines.extend(hoist_scripts_lines);
  let mut attrs: String = new_scripts
    .iter()
    .map(|item| item.attr.clone())
    .collect::<Vec<_>>()
    .join(" ")
    .trim()
    .to_string();

  if attrs != "" {
    attrs = format!(" {}", attrs);
  }

  let mut scripts: Vec<_> = Vec::new();

  if is_vue2 {
    scripts.push(format!("<script {}>", attrs));
    scripts.push(front_matter.clone());
    scripts.extend(script_lines.clone());
    scripts.push("export default { data() { return { frontmatter } } }".to_string());
    scripts.push("</script>".to_string())
  } else {
    scripts.push(format!("<script setup{}>", attrs));
    scripts.push(front_matter.clone());
    scripts.extend(script_lines.clone());
    scripts.push("</script>".to_string());
  }

  let template = format!("<template>\n{}\n</template>", html);

  let mut code: Vec<_> = vec![template.clone()];
  let scripts = scripts
    .iter_mut()
    .map(|script_line| script_line.trim().to_string())
    // .filter(|script_line| script_line != "")
    .collect::<Vec<_>>();
  code.extend(scripts);

  println!("code: {:?}", &code);
  vue_compile(&code.join("\n"), &file_name, &id)
}
