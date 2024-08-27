enum Mark {
  Clint,
  Server,
}

fn file_sign_default() -> FileSign {
  Mark::Clint
}

#[derive(Debug)]
struct Route {
  index: Option<bool>,
  path: String,
  component: Option<String>,
  error_boundary: Option<String>,
  lazy: Option<bool>,
  #[serde(file_sign_default)]
  mark: Option<Mark>,
  children: Vec<Route>,
}
