use farmfe_core::regex::Regex;

fn pattern() -> bool {
  let js_var_def_regex = Regex::new(r"(?:^|\s+)(?:let|const|var)\s+(?<var_name>[\w$]+)").unwrap();
  let js_class_def_regex = Regex::new(r"\bclass\s+(?<class_name>[\w$]+)").unwrap();
  let js_func_def_regex = Regex::new(r"(?:^|\s+)function\s+(?<func_name>[\w$]+)\s*\(").unwrap();
  let test_str = "import 'foo' from './bar.js';
exprot * as baz from './baz.js';

let x = 42;
const y = 'hello';
var z = true;
var s = 'world';

class MyClass {
  constructor() {
    this.a = 1;
  }
  
  myMethod() {
    console.log('Hello from MyClass');
  }
}

export { MyClass as ExportedClass };
export default function myFunction(arg) {
  return arg * 2;
}";

  for capture in js_var_def_regex.captures_iter(&test_str) {
    if let Some(var_name) = capture.name("var_name") {
      println!("Found variable: {}", var_name.as_str());
    }
  }

  for capture in js_class_def_regex.captures_iter(&test_str) {
    if let Some(class_name) = capture.name("class_name") {
      println!("Found class: {}", class_name.as_str());
    }
  }

  for capture in js_func_def_regex.captures_iter(&test_str) {
    if let Some(func_name) = capture.name("func_name") {
      println!("Found function: {}", func_name.as_str());
    }
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pattern() {
    let result = pattern();
  }
}
