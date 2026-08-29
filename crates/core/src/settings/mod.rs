// settings/mod.rs
use std::fs;

use crate::error::{HyraxError, HyraxResult};
mod defaults;
use defaults::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    #[serde(default = "default_refresh_interval")]
    refresh_intreval: u32,
    #[serde(default = "default_font_size")]
    font_size: u32,
    #[serde(default = "default_theme")]
    theme: String,
}

impl Settings {
    pub fn new() -> HyraxResult<Self> {
        let path = dirs::config_dir()
            .ok_or_else(|| HyraxError::EngineInit("Couldn't find the config folder".to_owned()))?;

        let hyraxql_dir = path.join("Hyraxql");
        let _ = fs::create_dir_all(&hyraxql_dir); // Ensure directory exists

        let settings_path = hyraxql_dir.join("settings.toml");

        if settings_path.exists() {
            // TODO: read and parse the existing file
            todo!("Load existing settings.toml");
        } else {
            // Instantiate with defaults
            let defaults = Self {
                refresh_intreval: default_refresh_interval(),
                font_size: default_font_size(),
                theme: default_theme(),
            };

            // Serialize and create the file
            let toml_string = toml::to_string(&defaults)
                .map_err(|_| HyraxError::EngineInit("Failed to serialize defaults".to_owned()))?;

            fs::write(&settings_path, toml_string)
                .map_err(|_| HyraxError::EngineInit("Failed to write settings.toml".to_owned()))?;

            Ok(defaults)
        }
    }
}
