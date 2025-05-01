// mod config_io;
// mod env_setup;
// mod utils;

use donna::{
    create_alias_group, create_project, define_project_type, env_setup::handle_args,
    update_alias_group, XDG,
};

fn main() {
    env_logger::init();
    handle_args(); // handles reset and setup commands

    // let start = std::time::Instant::now();
    let xdg = XDG::new(None, None, None);
    create_alias_group("test", "./test_root/test", false, &xdg).unwrap();
    define_project_type(
        "test",
        Some(vec!["test".to_string()]),
        Some("/Users/levirogalla/Projects/lib/cli-project-manager/lua/builder.lua"),
        Some("/Users/levirogalla/Projects/lib/cli-project-manager/lua/opener.lua"),
        false,
        &xdg,
    )
    .unwrap();
    // // create_lib("lib", "./test_root/lib", true, &xdg);
    create_project("testproj", Some("test"), None, None, false, &xdg).unwrap();
    // open_project("testproj", None, &xdg);

    update_alias_group("test", Some("test2"), Some("./test_root/newtest"), &xdg).unwrap();

    // let duration = start.elapsed(); // Measure elapsed time
    // println!("Elapsed time: {:.2?}", duration);
}
