use std::collections::HashSet;

#[derive(Debug)]
struct Route {
    path: Option<String>,
    index: bool,
}

fn build_routes_map(strings: Vec<String>, level: usize) -> Vec<Route> {
    let mut result: Vec<Route> = Vec::new();
    let mut internal_imports = String::new();

    let first_segments: HashSet<String> = strings
        .iter()
        .map(|s| {
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() > level {
                parts[level].replace("/route", "")
            } else {
                String::new()
            }
        })
        .filter(|s| !s.is_empty() && s != "tsx" && s != "lazy")
        .collect();

    if first_segments.is_empty() {
        return result;
    }

    let mut reversed_segments: Vec<String> = first_segments.into_iter().collect();
    reversed_segments.reverse();

    for segment in reversed_segments {
        let filtered_strings: Vec<&String> = strings
            .iter()
            .filter(|s| {
                let parts: Vec<&str> = s.split('.').collect();
                parts.len() > level && (parts[level] == segment || parts[level] == format!("{}/route", segment))
            })
            .collect();

        let route_path = segment
            .replace(r"\(([^)]*)\)\??$", "$1?")
            .replace(r"\$+$", "*")
            .replace(r"^\$", ":");

        if filtered_strings.is_empty() {
            continue;
        }

        let mut new_node = Route {
            path: None,
            index: false,
        };

        if route_path == "_index" {
            new_node.index = true;
        } else if !route_path.starts_with('_') {
            new_node.path = Some(route_path);
        }

        let page = filtered_strings.iter().find(|s| {
            s.ends_with(&format!("{}.tsx", segment)) || s.ends_with(&format!("{}/route.tsx", segment))
        });

        result.push(new_node);
    }

    result
}

fn main() {
    let strings = vec![
        "home.route.tsx".to_string(),
        "about.route.tsx".to_string(),
        "contact.route.tsx".to_string(),
    ];
    let level = 0;
    let routes = build_routes_map(strings, level);
    for route in routes {
        println!("{:?}", route);
    }
}
