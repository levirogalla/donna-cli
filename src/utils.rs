use std::env;
use std::path::{Path, PathBuf};

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
