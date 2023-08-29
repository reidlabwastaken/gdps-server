use std::collections::HashMap;

pub fn format(map: HashMap<i32, impl ToString>) -> String {
    let mut result = String::new();

    for (k, v) in map {
        result.push_str(&format!("{}:{}", k, v.to_string()));
        result.push(':');
    }

    if !result.is_empty() {
        result.pop();
    }

    return result;
}