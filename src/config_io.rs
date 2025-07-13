use super::utils::{types, XDG};
use crate::errors::{ConfigError, ProjectConfigError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{collections::HashMap, fs};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    default_lib: Option<types::LibraryName>,
    library_paths: Option<HashMap<types::LibraryName, String>>,
    alias_groups: Option<HashMap<types::AliasGroupName, AliasGroup>>,
    project_types: Option<HashMap<types::ProjectTypeName, ProjectType>>,
    builders_dir: Option<String>,
    openers_dir: Option<String>,

    builders_opener: Option<String>,
    openers_opener: Option<String>,
    config_opener: Option<String>,
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
    const REL_DATA_DIR: &'static str = "project_manager";

    // use dependency injection for xdg to allow for parellel testing (multiple instances of XDG and home env var names)
    pub fn load(path: Option<&str>, xdg: &XDG) -> Result<Config, ConfigError> {
        let contents = fs::read_to_string(
            path.map(PathBuf::from)
                .unwrap_or_else(|| Self::get_path(xdg)),
        )?;

        let mut config: Config = toml::from_str(&contents)?;

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
                    .join(Self::REL_DATA_DIR)
                    .join("projects")
                    .to_str()
                    .unwrap()
                    .to_string()
            });

        config.builders_dir.get_or_insert_with(|| {
            PathBuf::from(xdg.get_data_home())
                .join(Self::REL_DATA_DIR)
                .join("builders")
                .to_str()
                .unwrap()
                .to_string()
        });

        config.openers_dir.get_or_insert_with(|| {
            PathBuf::from(xdg.get_data_home())
                .join(Self::REL_DATA_DIR)
                .join("openers")
                .to_str()
                .unwrap()
                .to_string()
        });

        Ok(config)
    }

    pub fn get_path(xdg: &XDG) -> PathBuf {
        PathBuf::from(xdg.get_config_home()).join(Self::RC_REL_PATH)
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
        if default {
            self.set_default_lib(name);
        }
    }

    pub fn delete_lib(&mut self, name: &str) -> Option<String> {
        self.library_paths.as_mut().unwrap().remove(name)
    }

    pub fn set_default_lib(&mut self, name: types::LibraryName) {
        self.default_lib = Some(name.to_string());
    }

    /// Get the default library path, if name is none, it will try to return the default library path
    pub fn get_lib_path(&self, name: Option<&str>) -> Option<&str> {
        self.library_paths
            .as_ref()
            .unwrap()
            .get(name.unwrap_or(self.default_lib.as_ref().map_or("default", |s| s.as_str())))
            .map(|s| s.as_str())
    }

    pub fn add_project_type(
        &mut self,
        name: types::ProjectTypeName,
        default_alias_groups: Option<Vec<types::AliasGroupName>>,
        builder: Option<&str>,
        opener: Option<&str>,
    ) {
        let builder_path_prefix = PathBuf::from(self.builders_dir.as_deref().unwrap_or(""));
        let opener_path_prefix = PathBuf::from(self.openers_dir.as_deref().unwrap_or(""));

        let builder = builder.map(|s| builder_path_prefix.join(s).to_str().unwrap().to_string());
        let opener = opener.map(|s| opener_path_prefix.join(s).to_str().unwrap().to_string());

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

    pub fn delete_project_type(&mut self, name: &str) -> Option<ProjectType> {
        self.project_types.as_mut().unwrap().remove(name)
    }

    pub fn get_project_type(&self, name: types::ProjectTypeName) -> Option<&ProjectType> {
        self.project_types
            .as_ref()
            .and_then(|project_types| project_types.get(&name))
    }

    pub fn get_libs(&self) -> Option<HashMap<types::LibraryName, String>> {
        self.library_paths.clone()
    }

    pub fn set_builders_path_prefix(&mut self, path: &str) {
        self.builders_dir = Some(path.to_string());
    }

    pub fn set_openers_path_prefix(&mut self, path: &str) {
        self.openers_dir = Some(path.to_string());
    }

    pub fn get_alias_groups(&self) -> Option<HashMap<types::AliasGroupName, AliasGroup>> {
        self.alias_groups.clone()
    }

    pub fn get_project_types(&self) -> Option<HashMap<types::ProjectTypeName, ProjectType>> {
        self.project_types.clone()
    }

    pub fn get_default_lib(&self) -> Option<types::LibraryName> {
        self.default_lib.clone()
    }

    pub fn get_builders_opener(&self) -> Option<String> {
        self.builders_opener.clone()
    }

    pub fn get_openers_opener(&self) -> Option<String> {
        self.openers_opener.clone()
    }

    pub fn get_config_opener(&self) -> Option<String> {
        self.config_opener.clone()
    }

    pub fn get_openers_path_prefix(&self) -> String {
        self.openers_dir.clone().unwrap() // will never be None if the config is loaded correctly
    }

    pub fn get_builders_path_prefix(&self) -> String {
        self.builders_dir.clone().unwrap() // will never be None if the config is loaded correctly
    }
}

impl AliasGroup {
    pub fn new(path: &str) -> AliasGroup {
        AliasGroup {
            path: path.to_string(),
        }
    }

    pub fn get_project_configs(&self) -> Result<Vec<ProjectConfig>, std::io::Error> {
        let project_alias_configs: Vec<ProjectConfig> = fs::read_dir(&self.path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_dir() && path.is_symlink() {
                    Some(
                        ProjectConfig::load(
                            path.join(ProjectConfig::PROJECT_ROOT_REL_PATH)
                                .to_str()
                                .unwrap(),
                        )
                        .unwrap(),
                    )
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

    pub fn load(path: &str) -> Result<ProjectConfig, ProjectConfigError> {
        let contents = fs::read_to_string(path)?;

        let config: ProjectConfig = toml::from_str(&contents)?;

        Ok(config)
    }

    pub fn save(&self, path: &str) -> Result<(), ProjectConfigError> {
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
