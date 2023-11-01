use regex::Regex;

pub fn clean_no_space(string: &str) -> String {
    let regex = Regex::new(r"[^a-zA-z0-9_-]").unwrap();
    return regex.replace_all(string, "").to_string();
}

pub fn clean_basic(string: &str) -> String {
    let regex = Regex::new(r"[^A-Za-z0-9\-_ ]").unwrap();
    return regex.replace_all(string, "").to_string();
}

pub fn clean_char(string: &str) -> String {
    let regex = Regex::new(r"[^A-Za-z0-9 ]").unwrap();
    return regex.replace_all(string, "").to_string();
}