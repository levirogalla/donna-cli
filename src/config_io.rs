use super::utils::{types, XDG};
use clap::builder;
use serde::{Deserialize, Serialize};
use std::default;
use std::path::{PathBuf, Path};
use std::{collections::HashMap, error::Error, fs};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    default_lib: Option<types::LibraryName>,
    library_paths: Option<HashMap<types::LibraryName, String>>,
    alias_groups: Option<HashMap<types::AliasGroupName, AliasGroup>>,
    project_types: Option<HashMap<types::ProjectTypeName, ProjectType>>,
    builders_path_prefix: Option<String>,
    openers_path_prefix: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AliasGroup {
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectType {
    pub default_alias_groups: Option<Vec<types::AliasGroupName>>,
    pub builder: Option<String>,
    pub opener: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectConfig {
    pub project_type: Option<types::ProjectTypeName>,
    pub opener: Option<String>,
    pub builder: Option<String>,
    pub tracked_alias_groups: Option<Vec<types::AliasGroupName>>,
}

impl Config {
    const RC_REL_PATH: &'static str = "project_manager/config.toml";

    // use dependency injection for xdg to allow for parellel testing (multiple instances of XDG and home env var names)
    pub fn load(path: Option<&str>, xdg: &XDG) -> Result<Config, ConfigError> {
        let contents = fs::read_to_string(
            path.map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(xdg.get_config_home()).join(Self::RC_REL_PATH)),
        )?;

        let mut config: Config = toml::from_str(&contents).unwrap();

        config.library_paths.get_or_insert_with(HashMap::new);
        config.alias_groups.get_or_insert_with(HashMap::new);

        // set default library path if not set
        config
            .library_paths
            .as_mut()
            .unwrap()
            .entry("default".to_string())
            .or_insert_with(|| {
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

    pub fn add_alias_group(&mut self, name: types::AliasGroupName, alias: &AliasGroup) {
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

    pub fn get_alias_group(&self, name: &str) -> Option<&AliasGroup> {
        self.alias_groups.as_ref().unwrap().get(name)
    }

    pub fn delete_alias_group(&mut self, name: &str) -> Option<AliasGroup> {
        self.alias_groups.as_mut().unwrap().remove(name)
    }

    pub fn add_lib(&mut self, name: types::LibraryName, path: &str, default: bool) {
        self.library_paths
            .as_mut()
            .unwrap()
            .insert(name.to_string(), path.to_string());
    }

    pub fn set_default_lib(&mut self, name: types::LibraryName) {
        self.default_lib = Some(name.to_string());
    }

    pub fn get_lib_path(&self, name: Option<&str>) -> Result<&str, ConfigError> {
        self.library_paths
            .as_ref()
            .expect("No library paths found")
            .get(name.unwrap_or(self.default_lib.as_ref().map_or("default", |s| s.as_str())))
            .map(|s| s.as_str())
            .ok_or(ConfigError {
                message: "Could not find library path".to_string(),
            })
    }

    pub fn add_project_type(
        &mut self,
        name: types::ProjectTypeName,
        default_alias_groups: Option<Vec<types::AliasGroupName>>,
        builder: Option<&str>,
        opener: Option<&str>,
    ) {
        let builder_path_prefix = PathBuf::from(self.builders_path_prefix.as_deref().unwrap_or(""));
        let opener_path_prefix = PathBuf::from(self.openers_path_prefix.as_deref().unwrap_or(""));
        
        let builder = builder
            .map(|s| builder_path_prefix.join(s).to_str().unwrap().to_string());
        let opener = opener
            .map(|s| opener_path_prefix.join(s).to_str().unwrap().to_string());

        match self.project_types {
            Some(ref mut project_types) => {
                // lazy load alias_groups
                project_types.insert(
                    name.to_string(),
                    ProjectType::new(default_alias_groups, builder.as_deref(), opener.as_deref()),
                );
            }
            None => {
                let mut project_types = HashMap::new();
                project_types.insert(
                    name.to_string(),
                    ProjectType::new(default_alias_groups, builder.as_deref(), opener.as_deref()),
                );
                self.project_types = Some(project_types);
            }
        }
    }

    pub fn get_project_type(&self, name: types::ProjectTypeName) -> Option<&ProjectType> {
        self.project_types
            .as_ref()
            .expect("Tried to get undefined project type")
            .get(&name)
    }
}

impl AliasGroup {
    pub fn new(path: &str) -> AliasGroup {
        AliasGroup {
            path: path.to_string(),
        }
    }

    pub fn get_project_configs(&self) -> Result<Vec<ProjectConfig>, ConfigError> {
        let project_alias_configs: Vec<ProjectConfig> = fs::read_dir(&self.path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_dir() && path.is_symlink() {
                    Some(ProjectConfig::load(path.join(ProjectConfig::PROJECT_ROOT_REL_PATH).to_str().unwrap()).unwrap())
                } else {
                    None
                }
            })
            .collect();
        Ok(project_alias_configs)
    }
}

impl ProjectType {
    pub fn new(
        default_alias_groups: Option<Vec<String>>,
        builder: Option<&str>,
        opener: Option<&str>,
    ) -> ProjectType {
        ProjectType {
            default_alias_groups,
            builder: builder.map(|s| s.to_string()),
            opener: opener.map(|s| s.to_string()),
        }
    }
}

impl ProjectConfig {
    pub const PROJECT_ROOT_REL_PATH: &'static str = ".pm/project.toml";

    pub fn new(
        project_type: Option<types::ProjectTypeName>,
        opener: Option<String>,
        builder: Option<String>,
        tracked_alias_groups: Option<Vec<String>>,
    ) -> ProjectConfig {
        ProjectConfig {
            tracked_alias_groups,
            project_type,
            opener,
            builder,
        }
    }
    pub fn load(path: &str) -> Result<ProjectConfig, ConfigError> {
        let contents = fs::read_to_string(path)?;

        let config: ProjectConfig = toml::from_str(&contents).unwrap();

        Ok(config)
    }

    pub fn save(&self, path: &str) -> Result<(), ConfigError> {
        let toml_str = toml::to_string(self)?;

        fs::write(path, toml_str)?;

        Ok(())
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        ProjectConfig {
            project_type: None,
            opener: None,
            tracked_alias_groups: Some(vec![]),
            builder: None,
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
