use std::collections::HashMap;

pub fn format(map: HashMap<i32, impl ToString>) -> String {
    let mut sorted_keys: Vec<i32> = map.keys().copied().collect();
    sorted_keys.sort();

    let mut result = String::new();

    for key in sorted_keys {
        if let Some(val) = map.get(&key) {
            result.push_str(&format!("{}:{}:", key, val.to_string()));
        }
    }

    if result.ends_with(":") {
        result.pop();
    }

    return result;
}