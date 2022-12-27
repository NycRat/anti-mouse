use std::{collections::HashMap, str::FromStr};

use device_query::Keycode;
use serde_json::Value;

pub struct Config {
    pub keybinds: HashMap<String, Keycode>,
}

impl Config {
    pub fn get_default_config() -> Self {
        let keybinds: HashMap<String, Keycode> = [
            ("motions_on_key", Keycode::Escape),
            ("motions_off_key", Keycode::I),
            ("up", Keycode::K),
            ("down", Keycode::J),
            ("left", Keycode::H),
            ("right", Keycode::L),
            ("count_0", Keycode::Key0),
            ("count_1", Keycode::Key1),
            ("count_2", Keycode::Key2),
            ("count_3", Keycode::Key3),
            ("count_4", Keycode::Key4),
            ("scroll_up", Keycode::Y),
            ("scroll_down", Keycode::E),
            ("click", Keycode::Space),
            ("right_click_modifier", Keycode::LShift),
        ]
        .iter()
        .map(|(lhs, rhs)| (String::from(*lhs), *rhs))
        .collect();

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
                            let new_rhs = config_json[lhs].as_str().unwrap_or_default();
                            match Keycode::from_str(new_rhs) {
                                Ok(key) => {
                                    println!("Set {} to {}", lhs, key);

                                    return (lhs.clone(), key);
                                }
                                Err(err) => {
                                    // device_query from_str is missing Key0
                                    if new_rhs == "Key0" {
                                        return (lhs.clone(), Keycode::Key0);
                                    } else {
                                        println!("{}: {:?}", new_rhs, err);
                                    }
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
