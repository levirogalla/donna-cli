use std::io::Write;

use clap::{Parser, Subcommand};
use donna::{
    create_alias_group, create_lib, create_project, define_project_type, env_setup, get_projects,
};
use env_logger;

/// Hi, I'm Donna, the best project manager, ever!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
enum Commands {
    /// Create a new project, library, alias group, or project type
    Create {
        #[command(subcommand)]
        entity: CreateEntity,
    },

    /// List all projects, libraries, alias groups, or project types
    List {
        #[command(subcommand)]
        list: ListEntity,
    },

    /// Import a library and all projects in it
    Import {
        // Library name
        name: String,

        /// Path to the library directory
        path: String,

        /// Set the library as the default
        #[arg(short = 'd', long, default_value_t = false)]
        default: bool,

        /// Default type of all projects unless specified otherwise
        #[arg(short = 't', long)]
        project_type: Option<String>,

        /// Don't ask for confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Delete a project, library, alias group, project type
    Delete, // /// List all projects
    // List,
    // /// Open a project
    // Open {
    //     /// Name of the project
    //     name: String,
    // },
    Forget,
}

#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
enum CreateEntity {
    /// Create a new project
    Project {
        /// Name of the project
        name: String,

        /// Whether to create a new directory for the project or handoff an existing one to the pm
        #[arg(short = 'H', long, default_value_t = false)]
        handoff: bool,

        /// Type of the project
        #[arg(short = 't', long)]
        project_type: Option<String>,

        /// Alias group for the project
        #[arg(short = 'g', long)]
        alias_group: Option<String>,

        /// Library for the project
        #[arg(short = 'l', long)]
        library: Option<String>,
    },

    /// Create a new alias group
    AliasGroup {
        /// The internal name of the alias group
        name: String,

        /// Path to the alias group directory
        path: String,

        /// Whether to create a new directory for the group or handoff an existing one to the pm
        #[arg(short = 'H', long, default_value_t = false)]
        handoff: bool,
    },

    /// Create a new library
    Lib {
        /// The internal name of the library
        name: String,

        /// Path to the library directory
        path: String,

        /// Set the library as the default
        #[arg(short, long, default_value_t = false)]
        default: bool,

        /// Whether to create a new directory for the project or handoff an existing one to the pm
        #[arg(short = 'H', long, default_value_t = false)]
        handoff: bool,
    },

    /// Create a new project type
    ProjectType {
        /// Name of the project type and project directory name
        name: String,

        /// Names of default alias group for the project type
        default_groups: Option<Vec<String>>,

        /// Path to the opener for the project type
        #[arg(short, long)]
        opener: Option<String>,
        /// Path to the builder for the project type
        #[arg(short, long)]
        builder: Option<String>,

        #[arg(short, long, default_value_t = false)]
        redefine: bool,
    },
}

#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
enum ListEntity {
    /// List all projects
    Projects {
        /// Show project libraries
        #[arg(short = 'l', long, default_value_t = false)]
        libs: bool,

        /// Show project types
        #[arg(short = 't', long, default_value_t = false)]
        types: bool,

        /// Show project paths
        #[arg(short = 'p', long, default_value_t = false)]
        paths: bool,

        /// Show all data
        #[arg(short = 'a', long, default_value_t = false)]
        all: bool,
    },

    /// List all libraries
    Libraries {},

    /// List all alias groups
    AliasGroups {},

    /// List all project types
    ProjectTypes {},
}

fn main() {
    let args = Cli::parse();
    if args.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let xdg = donna::XDG::new(None);
    env_setup::setup_pm(&xdg);

    match &args.command {
        Commands::Create { entity } => match entity {
            CreateEntity::Project {
                name,
                handoff,
                project_type,
                alias_group,
                library,
            } => {
                create_project(
                    name,
                    project_type.as_deref(),
                    alias_group.as_deref(),
                    library.as_deref(),
                    *handoff,
                    &xdg,
                );
            }
            CreateEntity::AliasGroup {
                name,
                handoff,
                path,
            } => {
                create_alias_group(name, path.as_str(), *handoff, &xdg);
            }
            CreateEntity::Lib {
                name,
                path,
                default,
                handoff,
            } => {
                create_lib(name, path, *default, *handoff, &xdg);
            }
            CreateEntity::ProjectType {
                name,
                default_groups,
                opener,
                builder,
                redefine,
            } => {
                define_project_type(
                    name,
                    default_groups.as_ref().map(|v| v.clone()),
                    builder.as_deref(),
                    opener.as_deref(),
                    *redefine,
                    &xdg,
                );
            }
        },
        Commands::List { list } => match list {
            ListEntity::Projects {
                libs,
                paths,
                types,
                all,
            } => {
                let projects: Vec<(
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                )> = get_projects(&xdg)
                    .iter()
                    .map(|d| {
                        let project_name = Some(d.0.clone());
                        let project_type = if *types || *all {
                            Some(d.1 .0.clone())
                        } else {
                            None
                        };
                        let project_lib = if *libs || *all {
                            Some(d.1 .1.clone())
                        } else {
                            None
                        };
                        let project_path = if *paths || *all {
                            Some(d.1 .2.clone())
                        } else {
                            None
                        };
                        (project_name, project_type, project_lib, project_path)
                    })
                    .collect();

                // Prepare headers
                let mut headers = vec!["Name".to_string()];
                if *types || *all {
                    headers.push("Type".to_string());
                }
                if *libs || *all {
                    headers.push("Lib".to_string());
                }
                if *paths || *all {
                    headers.push("Path".to_string());
                }

                // Prepare rows as Vec<Vec<String>>
                let mut rows: Vec<Vec<String>> = Vec::new();
                for (name, project_type, project_lib, project_path) in &projects {
                    let mut row = vec![name.clone().unwrap_or_default()];
                    if *types || *all {
                        row.push(project_type.clone().unwrap_or_default());
                    }
                    if *libs || *all {
                        row.push(project_lib.clone().unwrap_or_default());
                    }
                    if *paths || *all {
                        row.push(project_path.clone().unwrap_or_default());
                    }
                    rows.push(row);
                }

                // Compute max width for each column
                let mut col_widths: Vec<usize> = headers
                    .iter()
                    .map(|h| h.len())
                    .collect();

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
                let sep_row: Vec<String> = col_widths
                    .iter()
                    .map(|w| "-".repeat(*w))
                    .collect();
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
            _ => {}
        },

        Commands::Import {
            name,
            path,
            default,
            project_type,
            yes,
        } => {
            create_lib(name, path, *default, true, &xdg);
            let dir_items = std::fs::read_dir(path).unwrap();
            for item in dir_items.flatten() {
                let path = item.path();
                if !path.is_dir() {
                    continue;
                }
                let project_name = path.file_name().unwrap().to_str().unwrap();

                let mut project_type = project_type.clone();
                if !*yes {
                    print!(
                        "Do you want to import the project '{}'? [y/N] ",
                        project_name
                    );
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().to_lowercase();
                    if input != "y" && input != "yes" {
                        continue;
                    }
                    print!(
                        "Project type for '{}' (default is {}): ",
                        project_name,
                        project_type.as_ref().unwrap_or(&"None".to_string())
                    );
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();
                    if !input.is_empty() {
                        project_type = Some(input.to_string());
                    }
                }
                create_project(
                    project_name,
                    project_type.as_deref(),
                    None,
                    Some(name),
                    true,
                    &xdg,
                );
            }
        }
        Commands::Delete => {
            // delete_project(name, &xdg);
        }

        Commands::Forget => {
            // forget_project(name, &xdg);
        }
    }
}
