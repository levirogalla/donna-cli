mod config_io;
mod env_setup;
mod utils; // re export for tests

use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

pub use utils::XDG;
pub use config_io::{Config, Alias};

pub fn create_alias_group(name: &str, path: &str, builder: Option<&str>, opener: Option<&str>, xdg: &XDG) {
    let mut config = Config::load(None, xdg).expect("Could not load config");
    fs::create_dir_all(path).expect("Could not create file");
    config.add_alias_group(name, &Alias::new(path, builder, opener));
    config.save(None, xdg).expect("Could not save config");
}

pub fn create_project(name: &str, alias: Option<&str>, lib: Option<&str>, xdg: &XDG) {
    let config = Config::load(None, xdg).expect("Could not load config");
    let lib_path = Path::new(config.get_lib_path(lib).expect("Could not find lib path")).join(name);
    fs::create_dir_all(lib_path.to_str().unwrap()).expect("Could not create project");
    
    if alias.is_none() { return }

    let alias = config.get_alias_group(alias.unwrap()).expect("Could not find alias");
    let alias_path = Path::new(&alias.path).join(name);
    symlink(lib_path, alias_path).expect("Could not create symlink");
    match &alias.builder {
        Some(_) => {
            todo!();
        }
        None => (),
    }
}

pub use env_setup::setup_pm;
