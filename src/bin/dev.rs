// mod config_io;
// mod env_setup;
// mod utils;

use cli_project_manager::{
    create_alias_group, create_lib, create_project, define_project_type, setup_pm, update_alias_group, AliasGroup, Config,
    ProjectConfig, XDG, env_setup::handle_args
};
use std::{
    env, fs, ops::Deref, path::{self, Path, PathBuf}
};


fn main() {
    handle_args(); // handles reset and setup commands

    // let start = std::time::Instant::now();
    let xdg = XDG::new(None);
    create_alias_group("test", "./test_root/test", false, &xdg);
    define_project_type("test", Some(vec!["test".to_string()]), Some("/Users/levirogalla/Projects/lib/cli-project-manager/lua/builder.lua"), Some("/Users/levirogalla/Projects/lib/cli-project-manager/lua/opener.lua"), &xdg);
    // // create_lib("lib", "./test_root/lib", true, &xdg);
    create_project("testproj", Some("test"), None, None, false, &xdg);
    // open_project("testproj", None, &xdg);

    update_alias_group("test", Some("test2"), Some("./test_root/newtest"), &xdg);

    // let duration = start.elapsed(); // Measure elapsed time
    // println!("Elapsed time: {:.2?}", duration);

}
