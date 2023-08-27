use regex::Regex;

pub fn clean(string: &str) -> String {
    let regex = Regex::new(r"[^a-zA-z0-9_-]").unwrap();
    return regex.replace_all(string, "").to_string();
}