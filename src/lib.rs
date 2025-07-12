//! # Project Manager
//!
//! A convenient library for managing a variety of project types, their configurations, and aliases.
//!
//! This library provides:
//! - **Project types**: Easily define default alias groups and commands for setting up or opening projects.
//! - **Alias groups**: Reference your projects from multiple places via symlinks.
//! - **Libraries**: Central storage for all projects, making them easy to find and manage.
//!

mod config_io;
pub mod env_setup;
pub mod errors;
pub mod utils; // re export for tests

use config_io::ProjectType;
use mlua::Lua;
use std::collections::HashMap;

#[cfg(unix)]
use std::os::unix::fs::symlink;

#[cfg(windows)]
use std::os::windows::fs::symlink_dir as symlink;

use std::path::{Path, PathBuf};
use std::{collections::HashSet, fs};
use utils::{delete, to_full_path};

pub use config_io::{AliasGroup, Config, ProjectConfig};
pub use utils::XDG;

mod api_types {
    pub type AliasName<'a> = &'a str;
    pub type ProjectTypeName<'a> = &'a str;
    pub type LibraryName<'a> = &'a str;
}

/// Defines a new project type with optional default alias groups, builder command,
/// and opener command.
///
/// # Arguments
/// - `name` – The unique name for the project type (e.g., "rust").
/// - `default_alias_groups` – Optional set of alias groups to be automatically added.
/// - `builder` – Optional command (like "cargo init") to set up the project.
/// - `opener` – Optional command (like "code") to open the project.
/// - `xdg` – Reference to the current XDG configuration.
///
/// # Example
/// ```rust
/// // ...existing code...
/// //define_project_type("my_project", None, Some("git init"), Some("code"), &xdg);
/// // ...existing code...
/// ```
pub fn define_project_type(
    name: &str,
    default_alias_groups: Option<Vec<String>>,
    builder: Option<&str>,
    opener: Option<&str>,
    redefine: bool,
    xdg: &XDG,
) -> Result<(), errors::ProjectTypeDefinitionError> {
    let mut config = Config::load(None, xdg)?;
    if !redefine && config.get_project_type(name.to_string()).is_some() {
        Err(errors::ProjectTypeAlreadyTrackedError(format!(
            "Project type {} already exists",
            name
        )))?
    } else if redefine && config.get_project_type(name.to_string()).is_none() {
        Err(errors::ProjectTypeNotTrackedError(format!(
            "Project type {} does not exist",
            name
        )))?
    } else {
        config.add_project_type(name.to_string(), default_alias_groups, builder, opener);
        config.save(None, xdg)?;
        Ok(())
    }
}

/// Creates a new alias group, ensuring the directory exists and registering it in the config.
///
/// # Arguments
/// - `name` – The name of the alias group.
/// - `path` – The path where the alias will be linked.
/// - `xdg` – The XDG configuration reference.
pub fn create_alias_group(
    name: &str,
    path: &str,
    already_exists: bool,
    xdg: &XDG,
) -> Result<(), errors::CreateAliasGroupError> {
    let mut config = Config::load(None, xdg)?;
    let path = &to_full_path(path);
    if !already_exists {
        if path.exists() {
            Err(errors::AliasGroupPathExistsError(format!(
                "Alias group already exists at {}",
                path.display()
            )))?;
        }
        if config.get_alias_group(name).is_some() {
            Err(errors::AliasGroupAlreadyTrackedError(format!(
                "Alias group {} already exists",
                name
            )))?;
        }
        fs::create_dir_all(path)?;
    }
    if !path.exists() {
        Err(errors::AliasGroupPathDoesNotExistError(format!(
            "Alias group path {} does not exist",
            path.display()
        )))?;
    }
    config.add_alias_group(name.to_string(), &AliasGroup::new(path.to_str().unwrap()));
    config.save(None, xdg)?;
    Ok(())
}

/// Creates a new library and optionally sets it as the default library.
///
/// # Arguments
/// - `name` – The identifier for the library.
/// - `path` – Filesystem path where projects will be stored.
/// - `default` – Whether to set this library as the default.
/// - `xdg` – XDG configuration reference.
pub fn create_lib(
    name: &str,
    path: &str,
    default: bool,
    already_exists: bool,
    xdg: &XDG,
) -> Result<(), errors::CreateLibError> {
    let mut config = Config::load(None, xdg)?;
    let path = &to_full_path(path);
    if !already_exists {
        if path.exists() {
            Err(errors::LibPathExistsError(format!(
                "Library already exists at {}",
                path.display()
            )))?;
        }
        fs::create_dir_all(path)?;
    }
    if !path.exists() {
        Err(errors::LibPathDoesNotExistError(format!(
            "Library path {} does not exist",
            path.display()
        )))?;
    }
    config.add_lib(name.to_string(), path.to_str().unwrap(), default);
    if default {
        config.set_default_lib(name.to_string());
    }
    config.save(None, xdg)?;
    Ok(())
}

/// Creates a new project, optionally specifying a project type, alias group, and library.
///
/// # Arguments
/// - `name` – The name of the new project.
/// - `project_type` – Optional project type to configure specific defaults.
/// - `alias_group` – Optional alias group to link the project to.
/// - `lib` – Optional library name to store the project in.
/// - `already_exists` – Optional flag to indicate if the project already exists. If it does, it will not call the builder and it will not create the project directory.
/// - `git_clone` – Optional git repository URL to clone the project from. This will block the builder from running.
/// - `xdg` – XDG configuration reference.
pub fn create_project(
    name: &str,
    project_type: Option<api_types::ProjectTypeName>,
    alias_groups: Option<&[api_types::AliasName]>,
    lib: Option<api_types::LibraryName>,
    already_exists: bool,
    git_clone: Option<&str>,
    xdg: &XDG,
) -> Result<(), errors::CreateProjectError> {
    // TODO: Allow just passing of alias location, maybe you want to make an alias not in a designated alias group, just in like a school folder for example
    let config = Config::load(None, xdg)?;
    let project_path = Path::new(config.get_lib_path(lib).ok_or(errors::LibNotTrackedError(
        format!(
            "Library '{}' could not be found",
            lib.unwrap_or("[default]")
        ),
    ))?)
    .join(name);
    let project_config_file_path = project_path.join(ProjectConfig::PROJECT_ROOT_REL_PATH);
    // let project_path = project_config_path.parent().expect("Invalid project config path");

    let project_config_dir = project_config_file_path.parent().unwrap();

    match (
        already_exists,
        project_config_dir.exists(),
        project_config_file_path.exists(),
        git_clone,
    ) {
        (false, false, false, None) => {
            fs::create_dir_all(project_config_dir)?;
            fs::File::create_new(&project_config_file_path)?;
        }
        (false, false, false, Some(git_clone)) => {

            let mut command = std::process::Command::new("git");
            command.arg("clone").arg(git_clone).arg(&project_path);
            log::info!("Running git clone: {:?}", command);
            let status = command.status()?;
            if !status.success() {
                Err(errors::SubProcessError(format!(
                    "Error running git clone: {}",
                    git_clone
                )))?;
            }
            if project_config_dir.exists() {
                log::error!("There was a .pm dir in the git repo, I don't know how to handle this yet :( Sorry.");
            } else {
                fs::create_dir_all(project_config_dir)?;
                fs::File::create_new(&project_config_file_path)?;
            }
        } // not possible
        (_, false, true, _) => {
            panic!("Something weird happened, project config file exists but not the directory");
        } // not possible
        (false, true, false, _) | (false, true, true, _) => {
            Err(errors::ProjectPathExistsError("Project config directory and/or file already exists, set already_exists to true if this is intended".to_string()))?;
        }
        (true, false, false, _) => {
            fs::create_dir(project_config_dir)?;
            fs::File::create_new(&project_config_file_path)?;
        }
        (true, true, false, _) => {
            fs::File::create_new(&project_config_file_path)?;
        }
        (true, true, true, _) => {
            ProjectConfig::load(project_config_file_path.to_str().unwrap())?;
        }
    }

    let mut project_config = ProjectConfig::default();

    let mut project_alias_groups: HashSet<&str> = HashSet::new();
    if let Some(ags) = alias_groups {
        project_alias_groups.extend(ags);
    }

    if let Some(pt) = project_type {
        let project_type_config =
            config
                .get_project_type(pt.to_string())
                .ok_or(errors::ProjectTypeNotTrackedError(format!(
                    "Project type {} does not exist",
                    pt
                )))?;

        project_config.project_type = Some(pt.to_string());
        project_config.opener = project_type_config.opener.clone();
        project_config.builder = project_type_config.builder.clone();

        if let Some(alias_groups) = &project_type_config.default_alias_groups {
            project_alias_groups.extend(alias_groups.iter().map(|s| s.as_str()));
        }

        // don't run builder if git clone is specified
        if project_type_config.builder.is_some() && git_clone.is_none() {
            let builder = project_type_config.builder.as_ref().unwrap();

            let lua = Lua::new();
            let globals = lua.globals();
            globals.set("PM_PROJECT_NAME", name).unwrap();
            globals
                .set("PM_PROJECT_PATH", project_path.to_str())
                .unwrap();
            globals.set("PM_PROJECT_TYPE", pt).unwrap();
            globals.set("PM_PROJECT_LIB", lib).unwrap();
            if !already_exists {
                lua.load(fs::read_to_string(builder).map_err(|_| {
                    errors::BuilderPathNotFoundError(format!(
                        "Builder path {} does not exist",
                        builder
                    ))
                })?)
                .exec()
                .expect("Failed to run project builder")
            };
            // TODO: maybe run clean up code here to delete the project dir if building it fails
        }
    }

    for alias_group in project_alias_groups {
        let alias =
            config
                .get_alias_group(alias_group)
                .ok_or(errors::AliasGroupNotTrackedError(format!(
                    "Alias group {} does not exist",
                    alias_group
                )))?;
        let alias_path = Path::new(&alias.path).join(name);
        project_config
            .tracked_alias_groups
            .as_mut()
            .unwrap()
            .push(alias_group.to_string());
        symlink(&project_path, alias_path)?;
    }
    project_config.save(project_config_file_path.to_str().unwrap())?;

    Ok(())
}

/// Opens a project by loading its configuration and executing the specified opener command.
///
/// # Arguments
/// - `name` – The name of the project to open.
/// - `lib` – Optional library name to locate the project.
/// - `xdg` – XDG configuration reference.
pub fn open_project(
    name: &str,
    lib: Option<&str>,
    xdg: &XDG,
) -> Result<(), errors::OpenProjectError> {
    let config = Config::load(None, xdg)?;
    let project_path = Path::new(config.get_lib_path(lib).ok_or(errors::LibNotTrackedError(
        format!("Library not found: {}", lib.unwrap_or("[default]")),
    ))?)
    .join(name);
    let project_config_path = project_path.join(ProjectConfig::PROJECT_ROOT_REL_PATH);
    if !project_path.exists() {
        Err(errors::ProjectPathDoesNotExistError(format!(
            "Project path {} does not exist",
            project_path.display()
        )))?;
    }
    let project_config = ProjectConfig::load(project_config_path.to_str().unwrap())?;
    if let Some(opener) = project_config.opener {
        let lua = Lua::new();
        let globals = lua.globals();
        globals.set("PM_PROJECT_NAME", name).unwrap();
        globals
            .set("PM_PROJECT_PATH", project_path.to_str().unwrap())
            .unwrap();
        globals
            .set("PM_PROJECT_TYPE", project_config.project_type)
            .unwrap();
        globals.set("PM_PROJECT_LIB", lib).unwrap();
        lua.load(fs::read_to_string(&opener).map_err(|_| {
            errors::OpenerPathNotFoundError(format!("Opener path {} does not exist", &opener))
        })?)
        .exec()
        .expect("Failed to run project opener");
    }
    Ok(())
}

pub fn get_project_path(
    name: &str,
    lib: Option<&str>,
    xdg: &XDG,
) -> Result<PathBuf, errors::GetProjectPathError> {
    let config = Config::load(None, xdg).expect("Could not load config");
    let path = Path::new(config.get_lib_path(lib).ok_or(errors::LibNotTrackedError(
        "Library not tracked".to_string(),
    ))?)
    .join(name);
    if !path.exists() {
        Err(errors::ProjectPathDoesNotExistError(format!(
            "Project path {} does not exist",
            path.display()
        )))?;
    }
    Ok(path)
}

/// To update alias group name and move alias group to a new location.
///
/// # Arguments
/// - `name` – The current name of the alias group.
/// - `new_name` – Optional new name for the alias group.
/// - `new_path` – Optional new path for the alias group.
/// - `xdg` – XDG configuration reference.
pub fn update_alias_group(
    name: &str,
    new_name: Option<&str>,
    new_path: Option<&str>,
    xdg: &XDG,
) -> Result<(), errors::UpdateAliasGroupError> {
    let mut config = Config::load(None, xdg)?;
    let new_path = new_path.map(to_full_path);
    let alias = config
        .delete_alias_group(name)
        .ok_or(errors::AliasGroupNotTrackedError(format!(
            "Alias group {} does not exist",
            name
        )))?;
    let old_path = PathBuf::from(alias.path);
    let updated_name = new_name.unwrap_or(name);
    let updated_path = new_path.as_ref().unwrap_or(&old_path);
    if old_path != *updated_path {
        fs::rename(&old_path, updated_path)?;
        log::info!(
            "Moved alias group from {} to {}",
            old_path.display(),
            updated_path.display()
        );
    }
    config.add_alias_group(
        updated_name.to_string(),
        &AliasGroup::new(updated_path.to_str().unwrap()),
    );
    config.save(None, xdg)?;
    Ok(())
}

/// Untrack an alias group by removing it from the config.
///
/// # Arguments
/// - `name` – The name of the alias group to untrack.
/// - `xdg` – XDG configuration reference.
pub fn untrack_alias_group(name: &str, xdg: &XDG) -> Result<(), errors::UntrackAliasGroupError> {
    let mut config = Config::load(None, xdg)?;
    config
        .delete_alias_group(name)
        .ok_or(errors::AliasGroupNotTrackedError(format!(
            "Alias group {} does not exist",
            name
        )))?;
    config.save(None, xdg)?;
    let project = get_projects(xdg)?;
    for (_, (_, _, path)) in project.iter() {
        let project_config_path = Path::new(path).join(ProjectConfig::PROJECT_ROOT_REL_PATH);
        let mut project_config = ProjectConfig::load(project_config_path.to_str().unwrap())?;
        if project_config
            .tracked_alias_groups
            .as_ref()
            .unwrap()
            .contains(&name.to_string())
        {
            log::info!("Deleting alias group from project {}", name);
            let mut new_alias_groups = project_config.tracked_alias_groups.clone().unwrap();
            new_alias_groups.retain(|x| x != name);
            project_config.tracked_alias_groups = Some(new_alias_groups);
            project_config.save(project_config_path.to_str().unwrap())?;
        }
    }

    for project_type in get_project_types(xdg)?.iter() {
        let mut new_alias_groups = project_type
            .1
            .default_alias_groups
            .clone()
            .unwrap_or_default();
        new_alias_groups.retain(|x| x != name);
        define_project_type(
            project_type.0,
            Some(new_alias_groups),
            project_type.1.builder.as_deref(),
            project_type.1.opener.as_deref(),
            true,
            xdg,
        )?;
    }
    Ok(())
}

/// Delete an alias group and move it to system trash.
///
/// # Arguments
/// - `name` – The name of the alias group to delete.
/// - `xdg` – XDG configuration reference.
pub fn delete_alias_group(name: &str, xdg: &XDG) -> Result<(), errors::DeleteAliasGroupError> {
    let config = Config::load(None, xdg)?;
    let alias = config
        .get_alias_group(name)
        .ok_or(errors::AliasGroupNotTrackedError(format!(
            "Alias group {} does not exist",
            name
        )))?;
    if Path::new(&alias.path).exists() {
        delete(&alias.path)?;
    }
    untrack_alias_group(name, xdg)?;
    Ok(())
}

/// Untrack a library
pub fn untrack_library(name: &str, xdg: &XDG) -> Result<(), errors::UntrackLibError> {
    let mut config = Config::load(None, xdg)?;
    config
        .delete_lib(name)
        .ok_or(errors::LibNotTrackedError(format!(
            "Library {} does not exist",
            name
        )))?;
    config.save(None, xdg)?;
    Ok(())
}

/// Untrack a project type
pub fn untrack_project_type(name: &str, xdg: &XDG) -> Result<(), errors::UntrackProjectTypeError> {
    let mut config = Config::load(None, xdg)?;
    config
        .delete_project_type(name)
        .ok_or(errors::ProjectTypeNotTrackedError(format!(
            "Project type {} does not exist",
            name
        )))?;
    config.save(None, xdg)?;

    for project in get_projects(xdg)?.iter() {
        let project_config_path =
            Path::new(&project.1 .2).join(ProjectConfig::PROJECT_ROOT_REL_PATH);
        let mut project_config = ProjectConfig::load(project_config_path.to_str().unwrap())?;
        if project_config.project_type.as_deref() == Some(name) {
            log::info!("Deleting project type from project {}", name);
            project_config.project_type = None;
            project_config.save(project_config_path.to_str().unwrap())?;
        }
    }
    Ok(())
}

/// Get all projects that are tracked by donna, in all libraries.
///
/// # Arguments
/// - `xdg` – XDG configuration reference.
/// # Returns
/// - A HashMap where the key is the project name and the value is a tuple containing (project type, lib, path)
pub fn get_projects(
    xdg: &XDG,
) -> Result<HashMap<String, (String, String, String)>, errors::GetProjectsError> {
    let config = Config::load(None, xdg)?;
    // project_name -> (project_type, lib, path)
    let mut all_projects_data: HashMap<String, (String, String, String)> = HashMap::new();

    for (lib_name, lib_path) in config
        .get_libs()
        .ok_or(errors::LibNotTrackedError("No libraries found".to_string()))?
        .iter()
    {
        let projects = Path::new(lib_path).read_dir()?.filter_map(|f| {
            f.as_ref()
                .unwrap()
                .file_type()
                .unwrap()
                .is_dir()
                .then(|| f.unwrap())
        });
        for project in projects {
            let project_name = project.file_name().to_string_lossy().to_string();
            let project_config_path = project.path().join(ProjectConfig::PROJECT_ROOT_REL_PATH);
            let project_config = match ProjectConfig::load(project_config_path.to_str().unwrap()) {
                Ok(config) => config,
                Err(e) => {
                    log::warn!("Failed to load project config for {}: {}", project_name, e);
                    continue;
                }
            };

            all_projects_data.insert(
                project_name,
                (
                    project_config
                        .project_type
                        .clone()
                        .unwrap_or("".to_string()),
                    lib_name.clone(),
                    project.path().to_str().unwrap().to_string(),
                ),
            );
        }
    }
    Ok(all_projects_data)
}

/// Get all libraries that are tracked by donna
///
/// # Arguments
/// - `xdg` – XDG configuration reference.
pub fn get_libraries(xdg: &XDG) -> Result<HashMap<String, String>, errors::GetLibsError> {
    let config = Config::load(None, xdg)?;
    Ok(config
        .get_libs()
        .ok_or(errors::LibNotTrackedError("No libraries found".to_string()))?)
}

/// Get all alias groups that are tracked by donna
///
/// # Arguments
/// - `xdg` – XDG configuration reference.
pub fn get_alias_groups(
    xdg: &XDG,
) -> Result<HashMap<String, AliasGroup>, errors::GetAliasGroupsError> {
    let config = Config::load(None, xdg)?;
    Ok(config
        .get_alias_groups()
        .ok_or(errors::AliasGroupNotTrackedError(
            "No alias groups found".to_string(),
        ))?)
}

/// Get all project types that are tracked by donna
///
/// # Arguments
/// - `xdg` – XDG configuration reference.
pub fn get_project_types(
    xdg: &XDG,
) -> Result<HashMap<String, ProjectType>, errors::GetProjectTypesError> {
    let config = Config::load(None, xdg)?;
    Ok(config.get_project_types().unwrap_or_else(HashMap::new))
}

pub fn set_builders_path_prefix(
    path: &str,
    xdg: &XDG,
) -> Result<(), errors::SetBuildersPathPrefixError> {
    if !Path::new(path).is_dir() {
        Err(errors::BuilderPathNotFoundError(format!(
            "Path {} does not exist",
            path
        )))?;
    }
    let mut config = Config::load(None, xdg)?;
    config.set_builders_path_prefix(to_full_path(path).to_str().unwrap());
    config.save(None, xdg)?;
    Ok(())
}

pub fn set_openers_path_prefix(
    path: &str,
    xdg: &XDG,
) -> Result<(), errors::SetOpenersPathPrefixError> {
    if !Path::new(path).is_dir() {
        Err(errors::OpenerPathNotFoundError(format!(
            "Path {} does not exist",
            path
        )))?;
    }
    let mut config = Config::load(None, xdg)?;
    config.set_openers_path_prefix(to_full_path(path).to_str().unwrap());
    config.save(None, xdg)?;
    Ok(())
}

pub fn set_default_lib(name: &str, xdg: &XDG) -> Result<(), errors::SetDefaultLibError> {
    let mut config = Config::load(None, xdg)?;
    if config.get_lib_path(Some(name)).is_none() {
        Err(errors::LibNotTrackedError(format!(
            "Library {} does not exist",
            name
        )))?;
    }
    config.set_default_lib(name.to_string());
    config.save(None, xdg)?;
    Ok(())
}

// BLOCKED: need to track aliases for each project in the project config since the system doesn't track it
// pub fn set_project_alias_groups(name: &str, lib: Option<api_types::LibraryName>, alias_groups: Vec<String>, xdg: &XDG) {
//     let mut config = Config::load(None, xdg).expect("Could not load config");
//     let project_path = Path::new(config.get_lib_path(lib).expect("Could not find lib path")).join(name);

// }

/// Setup up the the data diroctory and config directory.
pub use env_setup::setup_pm;
