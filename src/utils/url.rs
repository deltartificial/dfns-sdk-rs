// @dfns-sdk-rs/src/utils/url.rs

use std::collections::HashMap;

pub struct PathAndQueryParams {
    pub path: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

pub fn build_path_and_query(pattern: &str, params: &PathAndQueryParams) -> String {
    let mut path = pattern.to_string();
    
    let param_regex = regex::Regex::new(r":[a-zA-Z]+").expect("Invalid regex pattern");
    let params_to_replace: Vec<String> = param_regex
        .find_iter(pattern)
        .map(|m| m.as_str()[1..].to_string())
        .collect();

    for key in params_to_replace {
        if let Some(value) = params.path.get(&key) {
            let encoded_value = urlencoding::encode(value);
            path = path.replace(&format!(":{}", key), &encoded_value);
        }
    }

    let query_string: String = params.query
        .iter()
        .filter(|(_, value)| !value.is_empty())
        .map(|(key, value)| format!("{}={}", key, urlencoding::encode(value)))
        .collect::<Vec<_>>()
        .join("&");

    if query_string.is_empty() {
        path
    } else {
        format!("{}?{}", path, query_string)
    }
}