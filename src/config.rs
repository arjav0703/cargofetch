use crate::cli::{art_status, art_type};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub enable_art: bool,
    #[serde(default)]
    pub art_type: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enable_art: art_status(),
            art_type: art_type(),
        }
    }
}

use directories::ProjectDirs;

/// Returns the path to the configuration file.
fn config_path() -> std::path::PathBuf {
    let proj = ProjectDirs::from("github", "arjav0703", "cargofetch")
        .expect("couldn't find a home directory");
    proj.config_dir().join("config.toml")
}

use std::{fs, io};
use toml;

/// Loads the configuration from the config file, creating it with default values if it does not exist.
pub fn load_config() -> io::Result<Config> {
    let path = config_path();

    if !path.exists() {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let default = Config::default();
        let toml = toml::to_string_pretty(&default).expect("serialize default config");
        fs::write(&path, toml)?;
        return Ok(default);
    }

    let content = fs::read_to_string(&path)?;
    let cfg =
        toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(cfg)
}

/// Saves the configuration to the config file.
pub fn save_config(cfg: &Config) -> io::Result<()> {
    let toml = toml::to_string_pretty(cfg).expect("serialize config");
    fs::write(config_path(), toml)
}
