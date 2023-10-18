use std::fs;

use std::sync::LazyLock;

use toml::Table;

pub static CONFIG: LazyLock<Table> = LazyLock::new(|| {
    let toml_str = fs::read_to_string("config.toml").expect("error finding toml config");
    let config: Table = toml::from_str(toml_str.as_str()).expect("error parsing toml config");

    return config;
});

pub fn config_get(key: &str) -> Option<&toml::Value> {
    let this = &CONFIG;
    let mut current = this.get(key)?;
    for val in key.split(".").skip(1) {
        current = current.as_table()?.get(val)?;
    }
    Some(current)
}

pub fn config_get_with_default<T: serde::Deserialize<'static>>(key: &str, default: T) -> T {
    config_get(key)
        .and_then(|v| v.clone().try_into().ok())
        .unwrap_or(default)
}