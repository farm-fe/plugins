use super::Preset;
pub fn get_react_router_dom_preset() -> Preset {
  Preset {
    form: "react-router-dom".to_string(),
    imports: vec![
      "useLinkClickHandler".to_string(),
      "useSearchParams".to_string(),
      "Link".to_string(),
      "NavLink".to_string(),
      "Navigate".to_string(),
      "Outlet".to_string(),
      "Route".to_string(),
      "Routes".to_string(),
    ],
  }
}
