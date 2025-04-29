#![allow(dead_code)]

use std::fs;
use std::path::{PathBuf, Path};
use std::env;

use donna::{setup_pm, XDG};


/// A test directory that is deleted when it goes out of scope
pub struct TestDir {
  path: String,
  pub drop: bool
}

impl TestDir {
  /// Create a new test directory
  pub fn new(path: &str) -> Self {
      fs::create_dir_all(path).unwrap();
      TestDir {
          path: path.to_string(),
          drop: true,
      }
  }

  /// Mark an existing directory as a test directory
  pub fn mark(path: &str) -> Self {
      assert!(PathBuf::from(path).exists());
      TestDir {
          path: path.to_string(),
          drop: true,
      }
  }
}

impl Drop for TestDir {
  fn drop(&mut self) {
      if self.drop { fs::remove_dir_all(&self.path).unwrap() };
  }
}

pub fn gen_test_home_path(unique_name: &str) -> PathBuf {
  PathBuf::from(env::current_dir().unwrap().join("tests/test_home_dirs/").join(unique_name))
}

pub fn gen_test_config_home_path(unique_name: &str) -> PathBuf {
  PathBuf::from(gen_test_home_path(unique_name)).join(".config")
}

pub fn gen_test_data_home_path(unique_name: &str) -> PathBuf {
  PathBuf::from(gen_test_home_path(unique_name)).join(".local/share")
}

pub fn gen_test_alias_groups_path(unique_name: &str) -> PathBuf {
  PathBuf::from(gen_test_home_path(unique_name)).join("alias_groups")
}

pub fn set_home_env(unique_name: &str, xdg: &XDG) {
  env::set_var(
      &xdg.home_var_name,
      gen_test_home_path(unique_name).to_str().unwrap(),
  );
}

pub fn set_config_env(unique_name: &str) {
  env::set_var(
      "XDG_CONFIG_HOME",
      gen_test_config_home_path(unique_name).to_str().unwrap(),
  );
}

pub fn set_data_env(unique_name: &str) {
  env::set_var(
      "XDG_DATA_HOME",
      gen_test_data_home_path(unique_name).to_str().unwrap(),
  );
}

pub fn delete_home(unique_name: &str) {
  fs::remove_dir_all(format!(
      "/Users/levirogalla/Projects/lib/cli-project-manager/tests/home_{}",
      unique_name
  ))
  .unwrap_or_else(|_| {
      println!("Test home directory is already deleted.");
  });
}

pub fn setup_home(unique_name: &str, xdg: &XDG) -> TestDir {
  set_home_env(unique_name, xdg);
  setup_pm(xdg);
  TestDir::mark(gen_test_home_path(unique_name).to_str().unwrap())
}

pub fn print_fs(dir: &str) {
  fn print_dir(path: &Path, prefix: String) {
      if let Ok(entries) = fs::read_dir(path) {
          for entry in entries.flatten() {
              let path = entry.path();
              let file_name = entry.file_name().into_string().unwrap_or_default();
              println!("{}{}", prefix, file_name);
              if path.is_dir() {
                  print_dir(&path, format!("{}  ", prefix));
              }
          }
      }
  }

  let root = Path::new(dir);
  if root.exists() && root.is_dir() {
      println!("{}", dir);
      print_dir(root, String::from("  "));
  } else {
      println!("Directory does not exist: {}", dir);
  }
}
