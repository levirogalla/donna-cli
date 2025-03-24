mod config_io;
mod env_setup;
mod utils;

use env_setup::{handle_args, TEST_PROJECTS_PATH};
use std::path::Path;
use cli_project_manager::XDG;

use cli_project_manager::{create_alias_group, create_project};

fn main() {
    handle_args(); // handles reset and setup commands
    let xdg = XDG::new(None);
    create_alias_group(
        "NewAlias2",
        Path::new(TEST_PROJECTS_PATH)
            .join("NewAlias")
            .to_str()
            .unwrap(),
        None,
        None,
        &xdg,
    );

    create_project("First Project 2", Some("NewAlias2"), None, &xdg);
}
