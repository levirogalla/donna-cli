use super::utils::XDG;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{collections::HashMap, error::Error, fs, hash::Hash};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub library_paths: Option<HashMap<String, String>>,
    alias_groups: Option<HashMap<String, Alias>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Alias {
    pub path: String,
    pub builder: Option<String>,
    pub opener: Option<String>,
}

impl Config {
    const RC_REL_PATH: &'static str = "./project_manager/config.toml";

    // use dependency injection for xdg to allow for parellel testing (multiple instances of XDG and home env var names)
    pub fn load(path: Option<&str>, xdg: &XDG) -> Result<Config, ConfigError> {
        let contents = fs::read_to_string(
            path.map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(xdg.get_config_home()).join(Self::RC_REL_PATH)),
        )?;

        let mut config: Config = toml::from_str(&contents).unwrap();

        config.library_paths.get_or_insert_with(HashMap::new);
        config.alias_groups.get_or_insert_with(HashMap::new);

        let v = config.library_paths.as_mut().unwrap().entry("default".to_string()).or_insert_with(|| {
            PathBuf::from(xdg.get_data_home())
                .join("project_manager/projects")
                .to_str()
                .unwrap()
                .to_string()
        });

        Ok(config)
    }

    pub fn save(&self, path: Option<&str>, xdg: &XDG) -> Result<(), ConfigError> {
        let toml_str = toml::to_string(self)?;

        fs::write(
            path.map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(xdg.get_config_home()).join(Self::RC_REL_PATH)),
            toml_str,
        )?;

        Ok(())
    }

    pub fn add_alias_group(&mut self, name: &str, alias: &Alias) {
        match self.alias_groups {
            Some(ref mut alias_groups) => {
                // lazy load alias_groups
                alias_groups.insert(name.to_string(), alias.clone());
            }
            None => {
                let mut alias_groups = HashMap::new();
                alias_groups.insert(name.to_string(), alias.clone());
                self.alias_groups = Some(alias_groups);
            }
        }
    }

    pub fn get_alias_group(&self, name: &str) -> Option<&Alias> {
        self.alias_groups.as_ref().unwrap().get(name)
    }

    pub fn get_lib_path(&self, name: Option<&str>) -> Result<&str, ConfigError> {
        self.library_paths
            .as_ref()
            .expect("No library paths found")
            .get(name.unwrap_or("default"))
            .map(|s| s.as_str())
            .ok_or(ConfigError {
                message: "Could not find library path".to_string(),
            })
    }

    pub fn add_lib(&mut self, name: &str, path: &str) {
        self.library_paths
            .as_mut()
            .unwrap()
            .insert(name.to_string(), path.to_string());
    }
}

impl Alias {
    pub fn new(path: &str, builder: Option<&str>, opener: Option<&str>) -> Alias {
        Alias {
            path: path.to_string(),
            builder: builder.map(|s| s.to_string()),
            opener: opener.map(|s| s.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct ConfigError {
    message: String,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
/// Implement `Error` trait so it can be used as a proper error type
impl Error for ConfigError {}

/// Implement `From` to allow automatic conversion from `io::Error`
impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError {
            message: format!("IO Error: {}", err),
        }
    }
}

/// Implement `From` to allow automatic conversion from `toml::de::Error`
impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError {
            message: format!("TOML Error: {}", err),
        }
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(err: toml::ser::Error) -> Self {
        ConfigError {
            message: format!("TOML Error: {}", err),
        }
    }
}
