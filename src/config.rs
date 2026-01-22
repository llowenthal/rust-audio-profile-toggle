use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProfileConfig {
    pub sink_id: i32,
    pub sink_label: String,
    pub sink_node_name: String,

    pub source_id: i32,
    pub source_label: String,
    pub source_node_name: String,

    pub sink_volume: f32,
    pub source_volume: f32,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            sink_id: 0,
            sink_label: "".to_string(),
            sink_node_name: "".to_string(),

            source_id: 0,
            source_label: "".to_string(),
            source_node_name: "".to_string(),

            sink_volume: 1.0,
            source_volume: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AppConfig {
    pub profile_a: ProfileConfig,
    pub profile_b: ProfileConfig,
    pub current_profile: String, // "A" or "B"
}

fn config_path() -> Result<PathBuf, String> {
    let base = dirs::config_dir().ok_or("Could not find config dir".to_string())?;
    Ok(base.join("rust-audio-profile-toggle").join("config.toml"))
}

pub fn load_config_file() -> AppConfig {
    let mut cfg = AppConfig::default();
    cfg.current_profile = "A".to_string();

    if let Ok(path) = config_path() {
        if let Ok(s) = fs::read_to_string(&path) {
            if let Ok(parsed) = toml::from_str::<AppConfig>(&s) {
                return parsed;
            }
        }
    }
    cfg
}

pub fn save_config_file(cfg: &AppConfig) -> Result<(), String> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir failed: {}", e))?;
    }
    let s = toml::to_string_pretty(cfg).map_err(|e| format!("toml encode failed: {}", e))?;
    fs::write(&path, s).map_err(|e| format!("write failed: {}", e))?;
    Ok(())
}
