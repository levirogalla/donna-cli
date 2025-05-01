use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::errors;

pub mod types {
    pub type AliasGroupName = String;
    pub type ProjectTypeName = String;
    pub type LibraryName = String;
}

pub struct XDG {
    pub home_var_name: String,
    pub config_home_name: String,
    pub data_home_name: String,
}
impl XDG {
    pub fn new(
        home_var_name: Option<&str>,
        config_home_name: Option<&str>,
        data_home_name: Option<&str>,
    ) -> Self {
        XDG {
            home_var_name: home_var_name.unwrap_or("HOME").to_string(),
            config_home_name: config_home_name.unwrap_or("XDG_CONFIG_HOME").to_string(),
            data_home_name: data_home_name.unwrap_or("XDG_DATA_HOME").to_string(),
        }
    }

    pub fn get_config_home(&self) -> String {
        match env::var(&self.config_home_name) {
            Ok(val) => val,
            Err(_) => {
                let home = env::var(&self.home_var_name).expect("Could not find HOME env variable");
                Path::new(&home)
                    .join(".config")
                    .to_str()
                    .unwrap()
                    .to_string()
            }
        }
    }

    pub fn get_data_home(&self) -> String {
        match env::var(&self.data_home_name) {
            Ok(val) => val,
            Err(_) => {
                println!("XDG_DATA_HOME not set, using default");
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

pub fn to_full_path(path: &str) -> PathBuf {
    let cwd = env::current_dir().unwrap();
    let full_path = Path::new(path);
    if full_path.is_absolute() {
        full_path.to_path_buf()
    } else {
        cwd.join(full_path)
    }
}

pub fn pretty_print_table(rows: Vec<Vec<String>>, headers: Vec<String>) {
    // Compute max width for each column
    let mut col_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            if cell.len() > col_widths[i] {
                col_widths[i] = cell.len();
            }
        }
    }

    // Print header
    let header_row: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:width$}", h, width = col_widths[i]))
        .collect();
    println!("{}", header_row.join(" | "));

    // Print separator
    let sep_row: Vec<String> = col_widths.iter().map(|w| "-".repeat(*w)).collect();
    println!("{}", sep_row.join("-|-"));

    // Print rows
    for row in rows {
        let padded_row: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(i, cell)| format!("{:width$}", cell, width = col_widths[i]))
            .collect();
        println!("{}", padded_row.join(" | "));
    }
}

/// Use trash to delete unless env var "DONNA_CLI_USE_TRASH" is set to "false"
pub fn delete(path: &str) -> Result<(), errors::DeleteError> {
    let use_trash = env::var("DONNA_CLI_USE_TRASH").unwrap_or_else(|_| "false".to_string());
    let path = Path::new(path);
    if use_trash == "false" {
        fs::remove_dir_all(path)?;
    } else {
        trash::delete(path)?;
    }
    Ok(())
}
