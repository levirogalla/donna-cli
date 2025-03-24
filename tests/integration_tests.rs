use cli_project_manager::{create_alias_group, create_project, setup_pm, XDG, Config, Alias};
use std::{
    env, fs,
    path::{self, Path, PathBuf},
};

const TEST_HOME: &str = "/Users/levirogalla/Projects/lib/cli-project-manager/tests/home";

/// A test directory that is deleted when it goes out of scope
struct TestDir {
    path: String,
}

impl TestDir {

    /// Create a new test directory
    fn new(path: &str) -> Self {
        fs::create_dir_all(path).unwrap();
        TestDir {
          path: path.to_string(),
        }
    }

    /// Mark an existing directory as a test directory
    fn mark(path: &str) -> Self {
        assert!(PathBuf::from(path).exists());
        TestDir {
          path: path.to_string(),
        }
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).unwrap();
    }
}

fn gen_test_home_path(unique_name: &str) -> PathBuf {
    PathBuf::from(format!("{TEST_HOME}_{unique_name}"))
}

fn gen_test_config_home_path(unique_name: &str) -> PathBuf {
    PathBuf::from(gen_test_home_path(unique_name)).join(".config")
}

fn gen_test_data_home_path(unique_name: &str) -> PathBuf {
    PathBuf::from(gen_test_home_path(unique_name)).join(".local/share")
}

fn gen_test_alias_groups_path(unique_name: &str) -> PathBuf {
    PathBuf::from(gen_test_home_path(unique_name)).join("alias_groups")
}

fn set_home_env(unique_name: &str, xdg: &XDG) {
    env::set_var(&xdg.home_var_name, gen_test_home_path(unique_name).to_str().unwrap());
}

fn set_config_env(unique_name: &str) {
    env::set_var(
        "XDG_CONFIG_HOME",
        gen_test_config_home_path(unique_name).to_str().unwrap(),
    );
}

fn set_data_env(unique_name: &str) {
    env::set_var(
        "XDG_DATA_HOME",
        gen_test_data_home_path(unique_name).to_str().unwrap(),
    );
}

fn delete_home(unique_name: &str) {
    fs::remove_dir_all(format!(
        "/Users/levirogalla/Projects/lib/cli-project-manager/tests/home_{}",
        unique_name
    ))
    .unwrap_or_else(|_| {
        println!("Test home directory is already deleted.");
    });
}

fn setup_home(unique_name: &str, xdg: &XDG) -> TestDir {
    set_home_env(unique_name, xdg);
    setup_pm(xdg);
    TestDir::mark(gen_test_home_path(unique_name).to_str().unwrap())
}

const TEST_ALIAS_PATH: &str =
    "/Users/levirogalla/Projects/lib/cli-project-manager/tests/home/groups";

#[test]
fn test_fs_setup() {
    let unique_name = "test_fs_setup";
    let xdg = XDG::new(Some(unique_name));
    
    let _cleanup = setup_home(unique_name, &xdg);

    assert!(gen_test_config_home_path(unique_name)
        .join("project_manager")
        .exists());
    assert!(gen_test_config_home_path(unique_name)
        .join("project_manager")
        .exists());

    drop(_cleanup);

    assert!(!gen_test_home_path(unique_name).exists());
}

#[test]
fn test_create_project_no_alias() {
    let unique_name = "test_create_project_no_alias";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);

    create_project("test-proj", None, None, &xdg);

    assert!(gen_test_data_home_path(unique_name)
        .join("project_manager/projects/test-proj")
        .exists());
}

#[test]
fn test_create_alias_group() {
    let unique_name = "test_create_alias_group";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);

    let alias_group_path = gen_test_alias_groups_path(unique_name).join("t-alias");
    create_alias_group("test-alias", alias_group_path.to_str().unwrap(), None, None, &xdg);

    assert!(alias_group_path.exists());
}

#[test]
fn test_create_project_with_alias() {
    let unique_name = "test_create_project_with_alias";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);

    let mut config = Config::load(None, &xdg).unwrap();
    config.add_lib("test-external-lib", gen_test_home_path(unique_name).join("lib").to_str().unwrap());
    config.save(None, &xdg).unwrap();

    let alias_group_path = gen_test_alias_groups_path(unique_name);
    create_alias_group("test-alias1", alias_group_path.join("a1").to_str().unwrap(), None, None, &xdg);
    create_alias_group("test-alias2", alias_group_path.join("a2").to_str().unwrap(), None, None, &xdg);
    
    create_project("test-proj1", Some("test-alias1"), None, &xdg);
    create_project("test-proj2", Some("test-alias1"), Some("test-external-lib"), &xdg);
    create_project("test-proj3", Some("test-alias2"), Some("test-external-lib"), &xdg);

    assert!(gen_test_data_home_path(unique_name)
        .join("project_manager/projects/test-proj1")
        .exists());
    assert!(gen_test_home_path(unique_name)
        .join("lib/test-proj3")
        .exists());
    assert!(gen_test_home_path(unique_name)
        .join("lib/test-proj2")
        .exists());


    assert!(alias_group_path.join("a1/test-proj1").exists());
    assert!(alias_group_path.join("a1/test-proj2").exists());
    assert!(alias_group_path.join("a2/test-proj3").exists());
    
}
