use std;
use std::env;
use std::fs;
use std::process;
use std::path::Path;
use log;

use super::utils::XDG;

pub const TEST_ROOT_PATH: &str = "/Users/levirogalla/Projects/lib/cli-project-manager/test_root";
pub const TEST_PROJECTS_PATH: &str =
    "/Users/levirogalla/Projects/lib/cli-project-manager/test_root/Projects";

enum FS {
    Folder,
    File,
}

pub fn get_config_path(xdg: &XDG) -> String {
    Path::new(&xdg.get_config_home()).join("project_manager").to_str().unwrap().to_string()
}

pub fn get_data_path(xdg: &XDG) -> String {
    Path::new(&xdg.get_data_home()).join("project_manager/projects").to_str().unwrap().to_string()
}

pub fn setup_pm(xdg: &XDG) {
    let config_dir = get_config_path(&xdg);
    let data_dir = get_data_path(&xdg); // don't know if im gonna need this yet
    let config_file_path = Path::new(&config_dir).join("config.toml");
    let directories: [(&str, &str, FS); 3] = [
        (&config_dir, "Config Root", FS::Folder),
        (&data_dir, "Data Root", FS::Folder),
        (config_file_path.to_str().unwrap(), "Aliases", FS::File),
    ];

    for (path, name, fs_type) in directories.iter().filter(|path| !Path::new(path.0).exists()) {
        let result = match fs_type {
            FS::Folder => fs::create_dir_all(path).map(|_| ()),
            FS::File => fs::File::create(path).map(|_| ()),
        };
        match result {
            Ok(_) => log::info!("{} path created at: {}", name, path),
            Err(e) => log::error!("Error creating {}: {}", name, e),
        }
    } 
}

fn reset_test_root() {
    // hard code check
    if TEST_ROOT_PATH != "/Users/levirogalla/Projects/lib/cli-project-manager/test_root" {
        panic!("TEST_ROOT_PATH is not set to expected value, aborting reset due to risk of deleting unintended files.");
    }

    let config_dir = format!("{}/.config", TEST_ROOT_PATH);
    let data_dir = format!("{}/.local/share", TEST_ROOT_PATH);

    let result = fs::remove_dir_all(TEST_ROOT_PATH)
        .and_then(|_| fs::create_dir_all(config_dir))
        .and_then(|_| fs::create_dir_all(data_dir));

    match result {
        Ok(_) => log::info!("Config directory removed"),
        Err(e) => log::error!("Error removing config directory: {}", e),
    }
}

pub fn handle_args() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("reset") => {
            reset_test_root();
            process::exit(0);
        }
        Some("setup") => {
            setup_pm(&XDG::new(None));
            process::exit(0);
        }
        Some("clean") => {
            reset_test_root();
            setup_pm(&XDG::new(None));
        }
        _ => {}
    }
}

