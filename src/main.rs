use clap::{Parser, Subcommand};
use cli_project_manager::{create_alias_group, create_project};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    command_arg: Option<String>,

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
    // /// List all projects
    // List,
    // /// Open a project
    // Open {
    //     /// Name of the project
    //     name: String,
    // },
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
        #[arg(short='t', long)]
        project_type: Option<String>,

        /// Alias group for the project
        #[arg(short='g', long)]
        alias_group: Option<String>,

        /// Library for the project
        #[arg(short='l', long)]
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
    },
}


fn main() {
    let args = Cli::parse();
    let xdg = cli_project_manager::XDG::new(None);

    match &args.command {
        Commands::Create { entity } => match entity {
            CreateEntity::Project {
                name,
                handoff,
                project_type,
                alias_group,
                library,
            } => {
                create_project(name.as_str(), project_type.as_deref(), alias_group.as_deref(), library.as_deref(), *handoff, &xdg);
            }
            CreateEntity::AliasGroup { name, handoff, path } => {
                create_alias_group(name.as_str(), path.as_str(), *handoff, &xdg);
                // Handle alias group creation
                println!("Creating alias group: {}", name);
                println!("Handoff: {}", handoff);
                println!("Path: {:?}", path);
            }
            CreateEntity::Lib { name, path, default, handoff } => {
                // Handle library creation
                println!("Creating library: {}", name);
                println!("Path: {}", path);
                println!("Default: {}", default);
                println!("Handoff: {}", handoff);
            }
            CreateEntity::ProjectType { name, default_groups: default_alias_group_names, opener, builder } => {
                // Handle project type creation
                println!("Creating project type: {}", name);
                println!("Default alias groups: {:?}", default_alias_group_names);
                println!("Opener: {:?}", opener);
                println!("Builder: {:?}", builder);
            }
        },
    }
    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name.as_ref().unwrap_or(&"World".to_string()));
    // }
}
