mod config_io;
mod env_setup;
mod utils;

use cli_project_manager::open_project;
use env_setup::{handle_args, TEST_PROJECTS_PATH};
use std::path::Path;
use std::fs;
use cli_project_manager::{define_project_type, XDG};
use mlua::prelude::*;
use mlua::Lua;
use cli_project_manager::{create_alias_group, create_project, create_lib, update_alias_group};

fn main() {
    handle_args(); // handles reset and setup commands

    // let start = std::time::Instant::now();
    let xdg = XDG::new(None);
    create_alias_group("test", "./test_root/test", &xdg);
    define_project_type("test", Some(vec!["test".to_string()]), Some("/Users/levirogalla/Projects/lib/cli-project-manager/lua/builder.lua"), Some("/Users/levirogalla/Projects/lib/cli-project-manager/lua/opener.lua"), &xdg);
    // // create_lib("lib", "./test_root/lib", true, &xdg);
    create_project("testproj", Some("test"), None, None, &xdg);
    // open_project("testproj", None, &xdg);

    update_alias_group("test", Some("test2"), Some("./test_root/newtest"), &xdg);

    // let duration = start.elapsed(); // Measure elapsed time
    // println!("Elapsed time: {:.2?}", duration);

}
