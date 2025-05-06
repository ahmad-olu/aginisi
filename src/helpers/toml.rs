use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{
    consts::FOLDER_NAME,
    model::toml_config::{AppConfig, AuthType, Config, Overview},
};

pub fn create_app_config() {
    if !Path::new(&format!("{}.toml", "aginisi_config")).exists() {
        let config = AppConfig {
            overview: Overview {
                name: "Aginisi".to_string(),
                version: 1,
            },
            config: Config {
                // auth: Some(AuthType::Jwt),
                auth: None,
                port: 3000,
            },
        };

        let mut file = File::create(format!("{}.toml", "aginisi_config")).unwrap();
        file.write(toml::to_string_pretty(&config).unwrap().as_bytes())
            .unwrap();
    }
}

pub fn read_app_config() -> AppConfig {
    let content = fs::read_to_string(&format!("{}.toml", "aginisi_config")).unwrap();
    return toml::from_str(&content).unwrap();
}
