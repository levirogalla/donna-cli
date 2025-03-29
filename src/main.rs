mod config_io;
mod env_setup;
mod utils;

use env_setup::{handle_args, TEST_PROJECTS_PATH};
use std::path::Path;
use cli_project_manager::{define_project_type, XDG};

use cli_project_manager::{create_alias_group, create_project, create_lib};

fn main() {
    handle_args(); // handles reset and setup commands
    let xdg = XDG::new(None);
    create_alias_group("test", "./test_root/test", &xdg);
    define_project_type("test", Some(vec!["test".to_string()]), None, None, &xdg);
    create_lib("lib", "./test_root/lib", true, &xdg);
    create_project("testproj", None, None, None, &xdg);

}
