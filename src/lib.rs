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
mod utils; // re export for tests

use mlua::prelude::*;
use mlua::Lua;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::{collections::HashSet, fs};
use trash;

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
    xdg: &XDG,
) {
    let mut config = Config::load(None, xdg).expect("Could not load config");
    config.add_project_type(name.to_string(), default_alias_groups, builder, opener);
    config.save(None, xdg).expect("Could not save config");
}

/// Creates a new alias group, ensuring the directory exists and registering it in the config.
///
/// # Arguments
/// - `name` – The name of the alias group.
/// - `path` – The path where the alias will be linked.
/// - `xdg` – The XDG configuration reference.
pub fn create_alias_group(name: &str, path: &str, already_exists: bool, xdg: &XDG) {
    let mut config = Config::load(None, xdg).expect("Could not load config");
    if already_exists && !Path::new(path).exists() {
        panic!("Library could not find library path");
    } else {
        fs::create_dir_all(path).expect("Could not create folder");
    }
    config.add_alias_group(name.to_string(), &AliasGroup::new(path));
    config.save(None, xdg).expect("Could not save config");
}

/// Creates a new library and optionally sets it as the default library.
///
/// # Arguments
/// - `name` – The identifier for the library.
/// - `path` – Filesystem path where projects will be stored.
/// - `default` – Whether to set this library as the default.
/// - `xdg` – XDG configuration reference.
pub fn create_lib(name: &str, path: &str, default: bool, already_exists: bool, xdg: &XDG) {
    let mut config = Config::load(None, &xdg).expect("Could not load config");
    if already_exists && !Path::new(path).exists() {
        panic!("Library could not find library path");
    } else {
        fs::create_dir_all(path).expect("Could not create folder");
    }
    config.add_lib(name.to_string(), path, default);
    if default {
        config.set_default_lib(name.to_string());
    }
    config.save(None, xdg).expect("Could not save config");
}

/// Creates a new project, optionally specifying a project type, alias group, and library.
///
/// # Arguments
/// - `name` – The name of the new project.
/// - `project_type` – Optional project type to configure specific defaults.
/// - `alias_group` – Optional alias group to link the project to.
/// - `lib` – Optional library name to store the project in.
/// - `already_exists` – Optional flag to indicate if the project already exists. If it does, it will not call the builder and it will not create the project directory.
/// - `xdg` – XDG configuration reference.
pub fn create_project(
    name: &str,
    project_type: Option<api_types::ProjectTypeName>,
    alias_group: Option<api_types::AliasName>,
    lib: Option<api_types::LibraryName>,
    already_exists: bool,
    xdg: &XDG,
) {
    // TODO: Allow just passing of alias location, maybe you want to make an alias not in a designated alias group, just in like a school folder for example
    let config = Config::load(None, xdg).expect("Could not load config");
    let project_path =
        Path::new(config.get_lib_path(lib).expect("Could not find lib path")).join(name);
    let project_config_file_path = project_path.join(ProjectConfig::PROJECT_ROOT_REL_PATH);
    // let project_path = project_config_path.parent().expect("Invalid project config path");

    let project_config_dir = project_config_file_path.parent().expect("Invalid project config path");
    if already_exists {
        fs::create_dir(project_config_dir).expect("Could not create project directory");
    } else {
        if project_path.exists() { panic!("Project already exists"); }
        fs::create_dir_all(project_config_dir).expect("Could not create project directory");
    }
    
    fs::File::create_new(&project_config_file_path).expect("Could not create project config file");

    let mut project_config = ProjectConfig::default();

    let mut project_alias_groups: HashSet<&str> = HashSet::new();
    if let Some(ag) = alias_group {
        project_alias_groups.insert(ag);
    }

    if let Some(pt) = project_type {
        let project_type_config = config
            .get_project_type(pt.to_string())
            .expect("Could not find project type");

        project_config.project_type = Some(pt.to_string());
        project_config.opener = project_type_config.opener.clone();
        project_config.builder = project_type_config.builder.clone();

        if let Some(alias_groups) = &project_type_config.default_alias_groups {
            project_alias_groups.extend(alias_groups.iter().map(|s| s.as_str()));
        }

        if let Some(builder) = &project_type_config.builder {
            let lua = Lua::new();
            let globals = lua.globals();
            globals.set("PM_PROJECT_NAME", name).unwrap();
            globals
                .set("PM_PROJECT_PATH", project_path.to_str())
                .unwrap();
            globals.set("PM_PROJECT_TYPE", pt).unwrap();
            globals.set("PM_PROJECT_LIB", lib).unwrap();
            if !already_exists { lua.load(fs::read_to_string(builder).expect("Could not find builder file"))
                .exec()
                .expect("Failed to run project builder") };
            // TODO: maybe run clean up code here to delete the project dir if building it fails
        }
    }

    for alias_group in project_alias_groups {
        let alias = config
            .get_alias_group(alias_group)
            .expect("Could not find alias");
        let alias_path = Path::new(&alias.path).join(name);
        project_config
            .tracked_alias_groups
            .as_mut()
            .unwrap()
            .push(alias_group.to_string());
        symlink(&project_path, alias_path).expect("Could not create symlink");
    }
    project_config
        .save(&project_config_file_path.to_str().unwrap())
        .expect("Could not save project config");
}

/// Opens a project by loading its configuration and executing the specified opener command.
///
/// # Arguments
/// - `name` – The name of the project to open.
/// - `lib` – Optional library name to locate the project.
/// - `xdg` – XDG configuration reference.
pub fn open_project(name: &str, lib: Option<&str>, xdg: &XDG) {
    let config = Config::load(None, xdg).expect("Could not load config");
    let project_path =
        Path::new(config.get_lib_path(lib).expect("Could not find lib path")).join(name);
    let project_config_path = project_path.join(ProjectConfig::PROJECT_ROOT_REL_PATH);
    let project_config = ProjectConfig::load(project_config_path.to_str().unwrap())
        .expect("Could not load project config");
    println!("Project config: {:?}", project_config);
    if let Some(opener) = project_config.opener {
        println!("sfsdfsdf");
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
        lua.load(fs::read_to_string(opener).expect("Could not find opener file"))
            .exec()
            .expect("Failed to run project opener");
    }
}

/// To update alias group name and move alias group to a new location.
///
/// # Arguments
/// - `name` – The current name of the alias group.
/// - `new_name` – Optional new name for the alias group.
/// - `new_path` – Optional new path for the alias group.
/// - `xdg` – XDG configuration reference.
pub fn update_alias_group(name: &str, new_name: Option<&str>, new_path: Option<&str>, xdg: &XDG) {
    let mut config = Config::load(None, xdg).expect("Could not load config");
    let alias = config
        .delete_alias_group(name)
        .expect("Could not find alias group");
    let old_path = alias.path;
    let updated_name = new_name.unwrap_or(name);
    let updated_path = new_path.unwrap_or(&old_path);
    if &old_path != updated_path {
        fs::rename(&old_path, updated_path).expect("Could not move alias group");
    }
    config.add_alias_group(updated_name.to_string(), &AliasGroup::new(updated_path));
    config.save(None, xdg).expect("Could not save config");
}

/// Untrack an alias group by removing it from the config.
///
/// # Arguments
/// - `name` – The name of the alias group to untrack.
/// - `xdg` – XDG configuration reference.
pub fn untrack_alias_group(name: &str, xdg: &XDG) {
    let mut config = Config::load(None, xdg).expect("Could not load config");
    config
        .delete_alias_group(name)
        .expect("Could not find alias group");
    config.save(None, xdg).expect("Could not save config");
}

/// Delete an alias group and move it to system trash.
///
/// # Arguments
/// - `name` – The name of the alias group to delete.
/// - `xdg` – XDG configuration reference.
pub fn delete_alias_group(name: &str, xdg: &XDG) {
    let mut config = Config::load(None, xdg).expect("Could not load config");
    let alias = config
        .get_alias_group(name)
        .expect("Could not find alias group");
    trash::delete(&alias.path).expect("Could not delete alias group");
    config
        .delete_alias_group(name)
        .expect("Could not find alias group");
    config.save(None, xdg).expect("Could not save config");
}

// BLOCKED: need to track aliases for each project in the project config since the system doesn't track it
// pub fn set_project_alias_groups(name: &str, lib: Option<api_types::LibraryName>, alias_groups: Vec<String>, xdg: &XDG) {
//     let mut config = Config::load(None, xdg).expect("Could not load config");
//     let project_path = Path::new(config.get_lib_path(lib).expect("Could not find lib path")).join(name);

// }

/// Setup up the the data diroctory and config directory.
pub use env_setup::setup_pm;
