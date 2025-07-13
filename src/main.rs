use std::io::Write;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell, ValueHint};
use donna::{
    create_alias_group, create_lib, create_project, define_project_type, env_setup,
    errors::{
        ConfigError, CreateAliasGroupError, CreateLibError, CreateProjectError,
        GetAliasGroupsError, GetLibsError, GetProjectPathError, GetProjectTypesError,
        GetProjectsError, OpenBuildersError, OpenConfigError, OpenOpenersError, OpenProjectError,
        ProjectTypeDefinitionError, UntrackAliasGroupError, UntrackLibError,
        UntrackProjectTypeError,
    },
    get_alias_groups, get_builders_path, get_config_path, get_libraries, get_openers_path,
    get_project_path, get_project_types, get_projects, open_builders, open_config, open_openers,
    open_project, set_builders_path_prefix, set_default_lib, set_openers_path_prefix,
    untrack_alias_group, untrack_library, untrack_project_type, utils, ProjectConfig,
};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Verbosity {
    Debug,
    Info,
    Warn,
    Error,
}

/// Hi, I'm Donna, the best file seceratery, ever!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Set the verbosity level (debug, info, warn, error)
    #[arg(long, value_enum, default_value_t = Verbosity::Info)]
    verbose: Verbosity,

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
        entity: ListEntity,
    },

    /// Import a library and all projects in it
    Import {
        // Library name
        name: String,

        /// Path to the library directory
        #[arg(value_hint = ValueHint::DirPath)]
        path: String,

        /// Set the library as the default
        #[arg(short = 'd', long, default_value_t = false)]
        default: bool,

        /// Only import new projects, ie projects that don't have a config file, defaults to true
        #[arg(short = 'n', long, default_value_t = true)]
        new: bool,

        /// Default type of all projects unless specified otherwise
        #[arg(short = 't', long)]
        project_type: Option<String>,

        /// Don't ask for confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Set configuration options
    Set {
        #[command(subcommand)]
        option: SetOption,
    },

    Open {
        #[command(subcommand)]
        entity: OpenEntity,
    },

    /// Delete a project, library, alias group, project type, not implemented yet
    Delete, // /// List all projects
    // List,
    // /// Open a project
    // Open {
    //     /// Name of the project
    //     name: String,
    // },
    /// Forget about an alias group, library, or project type, donna will no longer track it
    Forget {
        #[command(subcommand)]
        entity: ForgetEntity,
    },

    /// Generate shell completion scripts
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },

    #[command(hide = true, name = "_autocompletion-values")]
    AutocompletionValues {
        /// Which entity to get autocompletion values for
        #[arg(value_enum)]
        entity: String,

        /// Needed to know whizch projects to list
        library: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
enum CreateEntity {
    /// Create a new project
    Project {
        /// Name of the project
        #[arg(value_hint = ValueHint::Other)]
        name: String,

        /// Whether to create a new directory for the project or handoff an existing one to the pm
        #[arg(short = 'H', long, default_value_t = false)]
        handoff: bool,

        /// Type of the project
        #[arg(short = 't', long)]
        project_type: Option<String>,

        /// Alias group for the project
        #[arg(short = 'g', long, num_args(0..))]
        alias_groups: Option<Vec<String>>,

        /// Library for the project
        #[arg(short = 'l', long)]
        library: Option<String>,

        /// Url of git repository to clone, this overides the builder and conficts with handoff
        #[arg(short = 'u', long, value_hint = ValueHint::Url)]
        git_clone: Option<String>,
    },

    /// Create a new alias group
    AliasGroup {
        /// The internal name of the alias group
        #[arg(value_hint = ValueHint::Other)]
        name: String,

        /// Path to the alias group directory
        #[arg(value_hint = ValueHint::DirPath)]
        path: String,

        /// Whether to create a new directory for the group or handoff an existing one to the pm
        #[arg(short = 'H', long, default_value_t = false)]
        handoff: bool,
    },

    /// Create a new library
    Lib {
        /// The internal name of the library
        #[arg(value_hint = ValueHint::Other)]
        name: String,

        /// Path to the library directory
        #[arg(value_hint = ValueHint::DirPath)]
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
        #[arg(value_hint = ValueHint::Other)]
        name: String,

        /// Names of default alias group for the project type
        default_groups: Option<Vec<String>>,

        /// Path to the opener for the project type, this will be relative to the config variable `openers_dir`, the default is in the share directory   
        #[arg(short, long, value_hint = ValueHint::ExecutablePath)]
        opener: Option<String>,
        /// Path to the builder for the project type, this will be relative to the config variable `builders_dir`, the default is in the share directory 
        #[arg(short, long, value_hint = ValueHint::ExecutablePath)]
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

#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
enum SetOption {
    /// Set the default library
    DefaultLib {
        /// Name of the library
        name: String,
    },

    /// Builder path prefix
    BuildersPath {
        /// Path to the builders directory
        #[arg(value_hint = ValueHint::DirPath)]
        path: String,
    },

    /// Opener path prefix
    OpenersPath {
        /// Path to the openers directory
        #[arg(value_hint = ValueHint::DirPath)]
        path: String,
    },
}

#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
enum ForgetEntity {
    /// Forget about an alias group
    AliasGroup {
        /// Name of the alias group
        name: String,
    },

    /// Forget about a library
    Library {
        /// Name of the library
        name: String,
    },

    /// Forget about a project type
    ProjectType {
        /// Name of the project type
        name: String,
    },
}

#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
enum OpenEntity {
    /// Open a project
    Project {
        /// Name of the project
        name: String,

        /// Library to open the project with
        #[arg(short = 'l', long)]
        lib: Option<String>,

        /// Library to open the project with
        #[arg(short = 't', long, default_value_t = false)]
        terminal: bool,
    },

    /// Open the config file
    Config {
        /// Library to open the project with
        #[arg(short = 't', long, default_value_t = false)]
        terminal: bool,
    },

    /// Open the builders dir
    Builders {
        /// Library to open the project with
        #[arg(short = 't', long, default_value_t = false)]
        terminal: bool,
    },

    /// Open the openers dir
    Openers {
        /// Library to open the project with
        #[arg(short = 't', long, default_value_t = false)]
        terminal: bool,
    },
}

fn handle_config_error(error: ConfigError) {
    match error {
        ConfigError::BadPath(_) => {
            println!("Config file not found. Does it exist under ~/.config/donna/config.toml?");
        }
        ConfigError::TomlLoad(_) => {
            println!("Error parsing config file.");
        }
        ConfigError::TomlSave(_) => {
            println!("Error saving config file.");
        }
    }
}

fn main() {
    let args = Cli::parse();
    match args.verbose {
        Verbosity::Debug => {
            std::env::set_var("RUST_LOG", "debug");
        }
        Verbosity::Info => {
            std::env::set_var("RUST_LOG", "info");
        }
        Verbosity::Warn => {
            std::env::set_var("RUST_LOG", "warn");
        }
        Verbosity::Error => {
            std::env::set_var("RUST_LOG", "error");
        }
    }
    env_logger::init();

    let xdg = donna::XDG::new(None, None, None);
    env_setup::setup_pm(&xdg);

    match &args.command {
        Commands::Completion { shell } => {
            let mut app = Cli::command();
            let mut buf = Vec::new();
            generate(*shell, &mut app, "donna", &mut buf);
            let completion_script = String::from_utf8(buf).unwrap();

            // Read the custom completion script from file
            let custom_completion = match shell {
                Shell::Bash => {
                    include_str!("sh/bash_completion.sh")
                }
                Shell::Zsh => {
                    include_str!("sh/zsh_completion.sh")
                }
                _ => "",
            };

            // Print the original completion script plus our overrides
            println!("{completion_script}");
            print!("{custom_completion}");
        }

        Commands::AutocompletionValues { entity, library } => match entity.as_str() {
            "alias-groups" => {
                let groups = get_alias_groups(&xdg).unwrap_or_default();
                for (name, _) in groups {
                    println!("{name}");
                }
            }
            "libraries" => {
                let libs = get_libraries(&xdg).unwrap_or_default();
                for (name, _) in libs {
                    println!("{name}");
                }
            }
            "project-types" => {
                let types = get_project_types(&xdg).unwrap_or_default();
                for (name, _) in types {
                    println!("{name}");
                }
            }
            "projects" => {
                let projects = get_projects(&xdg).unwrap_or_default();
                for (name, (_, project_lib, _)) in projects {
                    if let Some(lib) = library {
                        if lib != &project_lib {
                            continue;
                        }
                    }
                    println!("{name}");
                }
            }
            _ => {}
        },

        Commands::Create { entity } => match entity {
            CreateEntity::Project {
                name,
                handoff,
                project_type,
                alias_groups,
                library,
                git_clone,
            } => {
                match create_project(
                    name,
                    project_type.as_deref(),
                    alias_groups
                        .as_ref()
                        .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>())
                        .as_deref(),
                    library.as_deref(),
                    *handoff,
                    git_clone.as_deref(),
                    &xdg,
                ) {
                    Ok(_) => {
                        println!("Project '{name}' created successfully.");
                    }
                    Err(CreateProjectError::ConfigError(config_error)) => {
                        handle_config_error(config_error);
                    }
                    Err(err) => {
                        println!("Error creating project: {err}");
                    }
                };
            }
            CreateEntity::AliasGroup {
                name,
                handoff,
                path,
            } => {
                match create_alias_group(name, path.as_str(), *handoff, &xdg) {
                    Ok(_) => {}
                    Err(CreateAliasGroupError::ConfigError(config_error)) => {
                        handle_config_error(config_error);
                    }
                    Err(err) => {
                        println!("Error creating alias group: {err}");
                    }
                };
            }
            CreateEntity::Lib {
                name,
                path,
                default,
                handoff,
            } => {
                match create_lib(name, path, *default, *handoff, &xdg) {
                    Ok(_) => {
                        println!("Library '{name}' created successfully.");
                    }
                    Err(CreateLibError::ConfigError(config_error)) => {
                        handle_config_error(config_error);
                    }
                    Err(err) => {
                        println!("Error creating library: {err}");
                    }
                };
            }
            CreateEntity::ProjectType {
                name,
                default_groups,
                opener,
                builder,
                redefine,
            } => {
                match define_project_type(
                    name,
                    default_groups.clone(),
                    builder.as_deref(),
                    opener.as_deref(),
                    *redefine,
                    &xdg,
                ) {
                    Ok(_) => {
                        println!("Project type '{name}' created successfully.");
                    }
                    Err(ProjectTypeDefinitionError::ConfigError(config_error)) => {
                        handle_config_error(config_error);
                    }
                    Err(err) => {
                        println!("Error creating project type: {err}");
                    }
                };
            }
        },

        Commands::List { entity: list } => match list {
            ListEntity::Projects {
                libs,
                paths,
                types,
                all,
            } => {
                let projects_map = match get_projects(&xdg) {
                    Ok(projects) => projects,
                    Err(GetProjectsError::ConfigError(config_error)) => {
                        handle_config_error(config_error);
                        return;
                    }
                    Err(err) => {
                        println!("Error getting projects: {err}");
                        return;
                    }
                };

                // Add this near the top of your file with other imports
                type ProjectRow = (
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                );
                let projects: Vec<ProjectRow> = projects_map
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

                utils::pretty_print_table(rows, headers);
            }

            ListEntity::Libraries {} => {
                let libs = match get_libraries(&xdg) {
                    Ok(libs) => libs,
                    Err(GetLibsError::ConfigError(err)) => {
                        handle_config_error(err);
                        return;
                    }
                    Err(err) => {
                        println!("Error getting libraries: {err}");
                        return;
                    }
                };
                let rows: Vec<Vec<String>> = libs
                    .iter()
                    .map(|(name, path)| vec![name.clone(), path.clone()])
                    .collect();
                let headers = vec!["Name".to_string(), "Path".to_string()];
                utils::pretty_print_table(rows, headers);
            }

            ListEntity::AliasGroups {} => {
                let alias_groups = match get_alias_groups(&xdg) {
                    Ok(groups) => groups,
                    Err(GetAliasGroupsError::ConfigError(err)) => {
                        handle_config_error(err);
                        return;
                    }
                    Err(err) => {
                        println!("Error getting alias groups: {err}");
                        return;
                    }
                };
                let rows: Vec<Vec<String>> = alias_groups
                    .iter()
                    .map(|(name, group)| vec![name.clone(), group.path.clone()])
                    .collect();
                let headers = vec!["Name".to_string(), "Path".to_string()];
                utils::pretty_print_table(rows, headers);
            }

            ListEntity::ProjectTypes {} => {
                let project_types = match get_project_types(&xdg) {
                    Ok(types) => types,
                    Err(GetProjectTypesError::ConfigError(err)) => {
                        handle_config_error(err);
                        return;
                    }
                };
                let rows: Vec<Vec<String>> = project_types
                    .iter()
                    .map(|(name, pt)| {
                        vec![
                            name.clone(),
                            pt.builder.clone().unwrap_or("".to_string()),
                            pt.opener.clone().unwrap_or("".to_string()),
                            pt.default_alias_groups
                                .clone()
                                .map(|v| v.join(", "))
                                .unwrap_or("".to_string()),
                        ]
                    })
                    .collect();
                let headers = vec![
                    "Name".to_string(),
                    "Builder".to_string(),
                    "Opener".to_string(),
                    "Default Groups".to_string(),
                ];
                utils::pretty_print_table(rows, headers);
            }
        },

        Commands::Import {
            name,
            path,
            default,
            new,
            project_type,
            yes,
        } => {
            match create_lib(name, path, *default, true, &xdg) {
                Ok(_) => {
                    println!("Library '{name}' created successfully.");
                }
                Err(CreateLibError::ConfigError(config_error)) => {
                    handle_config_error(config_error);
                    return;
                }
                Err(err) => {
                    println!("Error creating library: {err}");
                    return;
                }
            };
            let dir_items = std::fs::read_dir(path).unwrap();
            for item in dir_items.flatten() {
                let path = item.path();
                if !path.is_dir() {
                    continue;
                }
                let project_name = path.file_name().unwrap().to_str().unwrap();

                if *new
                    && ProjectConfig::load(
                        path.join(ProjectConfig::PROJECT_ROOT_REL_PATH)
                            .to_str()
                            .unwrap(),
                    )
                    .is_ok()
                {
                    println!("Project '{project_name}' already exists, skipping.");
                    continue;
                }

                let mut project_type = project_type.clone();
                if !*yes {
                    print!("Do you want to import the project '{project_name}'? [y/N] ");
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
                match create_project(
                    project_name,
                    project_type.as_deref(),
                    None,
                    Some(name),
                    true,
                    None,
                    &xdg,
                ) {
                    Ok(_) => {
                        println!("Project '{project_name}' created successfully.");
                    }
                    Err(CreateProjectError::ConfigError(config_error)) => {
                        handle_config_error(config_error);
                        return;
                    }
                    Err(err) => {
                        println!("Error creating project: {err}");
                        return;
                    }
                };
            }
        }

        Commands::Set { option } => match option {
            SetOption::DefaultLib { name } => {
                match set_default_lib(name, &xdg) {
                    Ok(_) => {
                        println!("Default library set to '{name}'");
                    }
                    Err(err) => {
                        println!("Error setting default library: {err}");
                    }
                };
            }
            SetOption::BuildersPath { path } => {
                match set_builders_path_prefix(path, &xdg) {
                    Ok(_) => {
                        println!("Builders path set to '{path}'");
                    }
                    Err(err) => {
                        println!("Error setting builders path: {err}");
                    }
                };
            }
            SetOption::OpenersPath { path } => {
                match set_openers_path_prefix(path, &xdg) {
                    Ok(_) => {
                        println!("Openers path set to '{path}'");
                    }
                    Err(err) => {
                        println!("Error setting openers path: {err}");
                    }
                };
            }
        },

        Commands::Open { entity } => match entity {
            OpenEntity::Project {
                name,
                lib,
                terminal,
            } => match terminal {
                true => {
                    let path = match get_project_path(name, lib.as_deref(), &xdg) {
                        Ok(path) => path,
                        Err(GetProjectPathError::ConfigError(config_error)) => {
                            handle_config_error(config_error);
                            return;
                        }
                        Err(err) => {
                            println!("Error getting project path: {err}");
                            return;
                        }
                    };
                    println!("{}", path.to_str().unwrap());
                }
                false => match open_project(name, lib.as_deref(), &xdg) {
                    Ok(_) => {
                        println!("Project '{name}' opened successfully.");
                    }
                    Err(OpenProjectError::ConfigError(config_error)) => {
                        handle_config_error(config_error);
                    }
                    Err(err) => {
                        println!("Error opening project: {err}");
                    }
                },
            },

            OpenEntity::Config { terminal } => {
                let print_config_path = || {
                    let path = get_config_path(&xdg);
                    println!("{}", path.to_str().unwrap());
                };

                match terminal {
                    true => print_config_path(),

                    false => match open_config(&xdg) {
                        Ok(_) => {
                            println!("Config opened successfully.");
                        }
                        Err(OpenConfigError::ConfigError(config_error)) => {
                            handle_config_error(config_error);
                        }
                        Err(OpenConfigError::ConfigVarNotDefined(var_name)) => {
                            println!("Config variable not defined: {var_name}, printing config path instead.");
                            print_config_path();
                        }
                        Err(err) => {
                            println!("Error opening config: {err}");
                        }
                    },
                }
            }

            OpenEntity::Builders { terminal } => {
                let print_builders_path = || match get_builders_path(&xdg) {
                    Ok(path) => {
                        println!("{}", path);
                    }
                    Err(config_error) => {
                        handle_config_error(config_error);
                    }
                };
                match terminal {
                    true => print_builders_path(),

                    false => match open_builders(&xdg) {
                        Ok(_) => {
                            println!("Config opened successfully.");
                        }
                        Err(OpenBuildersError::ConfigError(config_error)) => {
                            handle_config_error(config_error);
                        }
                        Err(OpenBuildersError::ConfigVarNotDefined(var_name)) => {
                            println!("Config variable not defined: {var_name}, printing builders path instead.");
                            print_builders_path();
                        }
                        Err(err) => {
                            println!("Error opening config: {err}");
                        }
                    },
                }
            }

            OpenEntity::Openers { terminal } => {
                let print_openers_path = || match get_openers_path(&xdg) {
                    Ok(path) => {
                        println!("{}", path);
                    }
                    Err(config_error) => {
                        handle_config_error(config_error);
                    }
                };
                match terminal {
                    true => print_openers_path(),

                    false => match open_openers(&xdg) {
                        Ok(_) => {
                            println!("Config opened successfully.");
                        }
                        Err(OpenOpenersError::ConfigError(config_error)) => {
                            handle_config_error(config_error);
                        }
                        Err(OpenOpenersError::ConfigVarNotDefined(var_name)) => {
                            println!("Config variable not defined: {var_name}, printing openers path instead.");
                            print_openers_path();
                        }
                        Err(err) => {
                            println!("Error opening config: {err}");
                        }
                    },
                }
            }
        },

        Commands::Delete => {
            // delete_project(name, &xdg);
        }

        Commands::Forget { entity } => match entity {
            ForgetEntity::AliasGroup { name } => match untrack_alias_group(name, &xdg) {
                Ok(_) => {
                    println!("Alias group '{name}' untracked successfully.");
                }
                Err(UntrackAliasGroupError::ConfigError(config_error)) => {
                    handle_config_error(config_error);
                }
                Err(err) => {
                    println!("Error untracking alias group: {err}");
                }
            },

            ForgetEntity::Library { name } => {
                let libraries = match get_libraries(&xdg) {
                    Ok(libraries) => libraries,
                    Err(GetLibsError::ConfigError(err)) => {
                        handle_config_error(err);
                        return;
                    }
                    Err(err) => {
                        println!("Error getting libraries: {err}");
                        return;
                    }
                };
                if libraries.contains_key(name) {
                    match untrack_library(name, &xdg) {
                        Ok(_) => {
                            println!("Library '{name}' untracked successfully.");
                        }
                        Err(UntrackLibError::ConfigError(config_error)) => {
                            handle_config_error(config_error);
                        }
                        Err(err) => {
                            println!("Error untracking library: {err}");
                        }
                    }
                } else {
                    println!("Library '{name}' not found.");
                }
            }

            ForgetEntity::ProjectType { name } => {
                let project_types = match get_project_types(&xdg) {
                    Ok(project_types) => project_types,
                    Err(GetProjectTypesError::ConfigError(err)) => {
                        handle_config_error(err);
                        return;
                    }
                };
                if project_types.contains_key(name) {
                    match untrack_project_type(name, &xdg) {
                        Ok(_) => {
                            println!("Project type '{name}' untracked successfully.");
                        }
                        Err(UntrackProjectTypeError::ConfigError(config_error)) => {
                            handle_config_error(config_error);
                        }
                        Err(err) => {
                            println!("Error untracking project type: {err}");
                        }
                    }
                } else {
                    println!("Project type '{name}' not found.");
                }
            }
        },
    }
}
