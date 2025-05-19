
#[derive(thiserror::Error, Debug)]
#[error("Sub process error: {0}")]
pub struct SubProcessError(pub String);

// Project errors
#[derive(thiserror::Error, Debug)]
#[error("Already tracked: {0}")]
pub struct ProjectAlreadyTrackedError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Not tracked: {0}")]
pub struct ProjectNotTrackedError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Not found: {0}")]
pub struct ProjectPathDoesNotExistError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Path exists: {0}")]
pub struct ProjectPathExistsError(pub String);

// Lib errors
#[derive(thiserror::Error, Debug)]
#[error("Already tracked: {0}")]
pub struct LibAlreadyTrackedError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Not tracked: {0}")]
pub struct LibNotTrackedError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Not found: {0}")]
pub struct LibPathDoesNotExistError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Path exists: {0}")]
pub struct LibPathExistsError(pub String);

// AliasGroup errors
#[derive(thiserror::Error, Debug)]
#[error("Already tracked: {0}")]
pub struct AliasGroupAlreadyTrackedError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Not tracked: {0}")]
pub struct AliasGroupNotTrackedError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Not found: {0}")]
pub struct AliasGroupPathDoesNotExistError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Path exists: {0}")]
pub struct AliasGroupPathExistsError(pub String);

// ProjectType errors
#[derive(thiserror::Error, Debug)]
#[error("Already tracked: {0}")]
pub struct ProjectTypeAlreadyTrackedError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Not tracked: {0}")]
pub struct ProjectTypeNotTrackedError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Not found: {0}")]
pub struct ProjectTypePathDoesNotExistError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Path exists: {0}")]
pub struct ProjectTypePathExistsError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Path not found: {0}")]
pub struct BuilderPathNotFoundError(pub String);

#[derive(thiserror::Error, Debug)]
#[error("Path not found: {0}")]
pub struct OpenerPathNotFoundError(pub String);

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    BadPath(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    TomlLoad(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSave(#[from] toml::ser::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectConfigError {
    #[error("IO error: {0}")]
    BadPath(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    TomlLoad(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSave(#[from] toml::ser::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectTypeDefinitionError {
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("Path already tracked: {0}")]
    AlreadyTracked(#[from] ProjectTypeAlreadyTrackedError),

    #[error("Path no tracked: {0}")]
    NotTrackec(#[from] ProjectTypeNotTrackedError),
}

#[derive(thiserror::Error, Debug)]
pub enum CreateAliasGroupError {
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("Alias group path exists: {0}")]
    PathExists(#[from] AliasGroupPathExistsError),

    #[error("Alias group path does not exist: {0}")]
    PathDoesNotExist(#[from] AliasGroupPathDoesNotExistError),

    #[error("Already tracked: {0}")]
    AlreadyTracked(#[from] AliasGroupAlreadyTrackedError),

    #[error("Error creating alias group: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum CreateLibError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // path exists
    #[error("Path exists: {0}")]
    PathExists(#[from] LibPathExistsError),

    // path does not exist
    #[error("Path does not exist: {0}")]
    PathDoesNotExist(#[from] LibPathDoesNotExistError),

    // io error
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum CreateProjectError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // project config error
    #[error("Project config error: {0}")]
    ProjectConfigError(#[from] ProjectConfigError),

    // lib not tracked
    #[error("Lib not tracked: {0}")]
    LibNotTracked(#[from] LibNotTrackedError),

    // io error
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    // project path exists
    #[error("Project path exists: {0}")]
    ProjectPathExists(#[from] ProjectPathExistsError),

    // project type not tracked error
    #[error("Project type not tracked: {0}")]
    ProjectTypeNotTracked(#[from] ProjectTypeNotTrackedError),

    // builder path not found
    #[error("Builder path not found: {0}")]
    BuilderPathNotFound(#[from] BuilderPathNotFoundError),

    // alias group not tracked
    #[error("Alias group not tracked: {0}")]
    AliasGroupNotTracked(#[from] AliasGroupNotTrackedError),

    // sub process error
    #[error("Sub process error: {0}")]
    SubProcessError(#[from] SubProcessError),
}

#[derive(thiserror::Error, Debug)]
pub enum OpenProjectError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // project config error
    #[error("Project config error: {0}")]
    ProjectConfigError(#[from] ProjectConfigError),

    // lib not tracked
    #[error("Lib not tracked: {0}")]
    LibNotTracked(#[from] LibNotTrackedError),

    // project path does not exist
    #[error("Project path does not exist: {0}")]
    ProjectPathDoesNotExist(#[from] ProjectPathDoesNotExistError),

    // opener path not found
    #[error("Opener path not found: {0}")]
    OpenerPathNotFound(#[from] OpenerPathNotFoundError),
}

#[derive(thiserror::Error, Debug)]
pub enum GetProjectPathError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // lib not tracked
    #[error("Lib not tracked: {0}")]
    LibNotTracked(#[from] LibNotTrackedError),

    // project path does not exist
    #[error("Project path does not exist: {0}")]
    ProjectPathDoesNotExist(#[from] ProjectPathDoesNotExistError),
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateAliasGroupError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // alias group not tracked
    #[error("Alias group not tracked: {0}")]
    AliasGroupNotTracked(#[from] AliasGroupNotTrackedError),

    // move file error
    #[error("Move file error: {0}")]
    MoveFileError(#[from] std::io::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum UntrackAliasGroupError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // alias group not tracked
    #[error("Alias group not tracked: {0}")]
    AliasGroupNotTracked(#[from] AliasGroupNotTrackedError),

    // project path does not exist
    #[error("Project path does not exist: {0}")]
    ProjectConfig(#[from] ProjectConfigError),

    // project type definition error
    #[error("Project type definition error: {0}")]
    ProjectTypeDefinitionError(#[from] ProjectTypeDefinitionError),

    // get projects error
    #[error("Get projects error: {0}")]
    GetProjectsError(#[from] GetProjectsError),

    // get project types error
    #[error("Get project types error: {0}")]
    GetProjectTypesError(#[from] GetProjectTypesError),
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteAliasGroupError {
    // config error`
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // alias group not tracked
    #[error("Alias group not tracked: {0}")]
    AliasGroupNotTracked(#[from] AliasGroupNotTrackedError),

    // trash error
    #[error("Trash error: {0}")]
    DeleteErrr(#[from] DeleteError),

    // untrack alias group error
    #[error("Untrack alias group error: {0}")]
    UntrackAliasGroupError(#[from] UntrackAliasGroupError),
}

#[derive(thiserror::Error, Debug)]
pub enum UntrackLibError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // lib not tracked
    #[error("Lib not tracked: {0}")]
    LibNotTracked(#[from] LibNotTrackedError),

    // project path does not exist
    #[error("Project path does not exist: {0}")]
    ProjectConfig(#[from] ProjectConfigError),

    // project type definition error
    #[error("Project type definition error: {0}")]
    ProjectTypeDefinitionError(#[from] ProjectTypeDefinitionError),
}

#[derive(thiserror::Error, Debug)]
pub enum UntrackProjectTypeError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // project type not tracked
    #[error("Project type not tracked: {0}")]
    ProjectTypeNotTracked(#[from] ProjectTypeNotTrackedError),

    // project path does not exist
    #[error("Project config error: {0}")]
    ProjectConfig(#[from] ProjectConfigError),

    // project type definition error
    #[error("Project type definition error: {0}")]
    ProjectTypeDefinitionError(#[from] ProjectTypeDefinitionError),

    // get projects error
    #[error("Get projects error: {0}")]
    GetProjectsError(#[from] GetProjectsError),
}

#[derive(thiserror::Error, Debug)]
pub enum GetProjectsError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // lib not tracked
    #[error("No libs are tracked: {0}")]
    LibNotTracked(#[from] LibNotTrackedError),

    // read dir error
    #[error("Read dir error: {0}")]
    ReadDirError(#[from] std::io::Error),

    // project config error
    #[error("Project config error: {0}")]
    ProjectConfigError(#[from] ProjectConfigError),
}

#[derive(thiserror::Error, Debug)]
pub enum GetLibsError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // lib not tracked
    #[error("Lib not tracked: {0}")]
    LibNotTracked(#[from] LibNotTrackedError),
}

#[derive(thiserror::Error, Debug)]
pub enum GetAliasGroupsError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // alias group not tracked
    #[error("Alias group not tracked: {0}")]
    AliasGroupNotTracked(#[from] AliasGroupNotTrackedError),
}

#[derive(thiserror::Error, Debug)]
pub enum GetProjectTypesError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),
}

#[derive(thiserror::Error, Debug)]
pub enum SetBuildersPathPrefixError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // path not found
    #[error("Path not found: {0}")]
    PathNotFound(#[from] BuilderPathNotFoundError),
}

#[derive(thiserror::Error, Debug)]
pub enum SetOpenersPathPrefixError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // path not found
    #[error("Path not found: {0}")]
    PathNotFound(#[from] OpenerPathNotFoundError),
}

#[derive(thiserror::Error, Debug)]
pub enum SetDefaultLibError {
    // config error
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    // lib not tracked
    #[error("Lib not tracked: {0}")]
    LibNotTracked(#[from] LibNotTrackedError),
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteError {
    // trash error
    #[error("Trash error: {0}")]
    TrashError(#[from] trash::Error),

    // io error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

// #[derive(thiserror::Error, Debug)]
// pub enum DonnaError {
//     #[error("Config error: {0}")]
//     ConfigError(#[from] ConfigError),

//     #[error("Path exists: {0}")]
//     PathExists(String),

//     #[error("Path does not exist: {0}")]
//     PathDoesNotExist(String),

//     #[error("Already tracked: {0}")]
//     AlreadyTracked(String),

//     #[error("Not tracked: {0}")]
//     NotTracked(String),

//     #[error("IO Error: {0}")]
//     IoError(#[from] std::io::Error),

//     #[error("Not found error: {0}")]
//     NotFoundError(#[from] NotFoundError),
// }
