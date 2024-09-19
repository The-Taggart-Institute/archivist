use serde::Deserialize;
use serde_json::from_str;
use std::fs::read_to_string;
use std::path::Path;

const CONFIG_FILENAME: &str = "config.json";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub wallabag_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub wallabag_username: String,
    pub wallabag_password: String
}

pub fn load_config() -> Result<Config, String> {
    let file_path = Path::new(CONFIG_FILENAME);
    if file_path.is_file() {
        let config_str = read_to_string(file_path).unwrap();
        return Ok(from_str(&config_str).unwrap());
        
    }
    Err("Config file not found".to_string())
}

#[test]
fn config_loads() {
    let config = load_config();
    println!("{:?}", config);
    assert!(config.is_ok());
}