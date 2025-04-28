use std::env;
use std::path::Path;

pub mod types {
  pub type AliasGroupName = String;
  pub type ProjectTypeName = String;
  pub type LibraryName = String;
}

pub struct XDG {
    pub home_var_name: String,
}
impl XDG {
    pub fn new(home_var_name: Option<&str>) -> Self {
        XDG {
            home_var_name: home_var_name.unwrap_or("HOME").to_string(),
        }
    }

    pub fn get_config_home(&self) -> String {
        match env::var("XDG_CONFIG_HOME") {
            Ok(val) => val,
            Err(_) => {
                let home = env::var(&self.home_var_name).expect("Could not find HOME variable");
                Path::new(&home)
                    .join(".config")
                    .to_str()
                    .unwrap()
                    .to_string()
            }
        }
    }

    pub fn get_data_home(&self) -> String {
        match env::var("XDG_DATA_HOME") {
            Ok(val) => val,
            Err(_) => {
                let home = env::var(&self.home_var_name).expect("Could not find HOME variable");
                Path::new(&home)
                    .join(".local/share")
                    .to_str()
                    .unwrap()
                    .to_string()
            }
        }
    }
}