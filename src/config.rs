use std::fs;

use std::sync::LazyLock;

use toml::Table;

use serde::de::DeserializeOwned;

pub static CONFIG: LazyLock<Table> = LazyLock::new(|| {
    let toml_str = fs::read_to_string("config.toml").expect("error finding toml config");
    let config: Table = toml::from_str(toml_str.as_str()).expect("error parsing toml config");

    return config;
});

pub fn config_get(key: &str) -> Option<&toml::Value> {
    let this = &CONFIG;
    let mut current_key = this.get(key.split(".").next()?)?;
    for val in key.split(".").skip(1) {
        current_key = current_key.get(val)?;
    }
    Some(current_key)
}

pub fn config_get_with_default<'de, T>(key: &str, default: T) -> T
where
    T: DeserializeOwned + 'de
{
    let val = config_get(key)
        .and_then(|v| v.to_owned().try_into().expect("invalid toml val"))
        .unwrap_or_else(|| default);

    return val;
}