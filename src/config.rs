use serde::Deserialize;
use std::fs;
use std::sync::LazyLock;

#[derive(Deserialize)]
pub struct Config {
    pub general: ConfigGeneral,
    pub accounts: ConfigAccounts,
    pub db: ConfigDB
}

#[derive(Deserialize)]
pub struct ConfigGeneral {
    pub append_path: String,
    pub port: u16
}

#[derive(Deserialize)]
pub struct ConfigAccounts {
    pub allow_registration: bool
}

#[derive(Deserialize)]
pub struct ConfigDB {
    pub data_folder: String
}

impl Config {
    pub fn load_from_file(file_path: &str) -> Self {
        let toml_str = fs::read_to_string(file_path).expect("Error finding toml config:");
        let config: Config = toml::from_str(toml_str.as_str()).expect("Error parsing toml config:");

        return config;
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config = Config::load_from_file("config.toml");
    
    return config;
});