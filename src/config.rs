use std::{collections::HashMap, str::FromStr};

use device_query::Keycode;
use serde_json::Value;

pub struct Config {
    pub keybinds: HashMap<String, Keycode>,
}

impl Config {
    pub fn get_default_config() -> Self {
        let mut keybinds = HashMap::new();
        keybinds.insert("motions_on_key".to_owned(), Keycode::Escape);
        keybinds.insert("motions_off_key".to_owned(), Keycode::I);
        Config { keybinds }
    }

    pub fn from_config_file(file: &str) -> Self {
        let config_data = std::fs::read_to_string(file).unwrap_or_default();
        let mut config = Self::get_default_config();
        match serde_json::from_str::<Value>(&config_data) {
            Ok(config_json) => {
                config.keybinds = config
                    .keybinds
                    .iter()
                    .map(|(lhs, rhs)| {
                        if config_json[lhs].is_string() {
                            match Keycode::from_str(config_json[lhs].as_str().unwrap_or_default()) {
                                Ok(key) => {
                                    // println!("Set {} to {}", lhs, key);

                                    return (lhs.clone(), key);
                                }
                                Err(err) => {
                                    println!("{:?}", err);
                                }
                            }
                        }
                        (lhs.clone(), *rhs)
                    })
                    .collect();
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
        return config;
    }
}
