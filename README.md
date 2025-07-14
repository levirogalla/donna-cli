# Donna - CLI Project Manager

[![CI Status](https://github.com/levirogalla/donna-cli/actions/workflows/ci.yaml/badge.svg)](https://github.com/levirogalla/donna-cli/actions/workflows/ci.yaml)

Donna is a powerful command-line project manager designed to help you organize, create, and manage projects across your system. It provides a structured way to manage project libraries, define project types, and simplify access to your work.

## Features

- **Project management**: Create, open, and organize projects
- **Library organization**: Donna encourages you to store all projects in one place called a library. The default location is `$XDG_DATA_HOME/project_manager/projects` or `~/.local/share/project_manager/projects`
- **Alias groups**: Organize projects by creating alias groups - for example, a group for WIP projects and a group for Python projects. This allows you to have the same project accessible from multiple locations on your file system. If you have C++ coding projects for a class, you could create an alias to your project in both your class folder and your C++ projects folder
- **Project types**: Define templates and behaviors for different kinds of projects. For example, a Python backend project type would automatically add a new project to the Python alias group and backends alias group. You can also define openers and builders for your project type - these are Lua scripts that are called on either `donna open project` or `donna create project`
- **Openers and Builders**: Custom scripting to be called for a project. For example, you could have the following code in the builder for Python projects:

```lua
os.execute("cd " .. PM_PROJECT_PATH .. " && python3 -m venv env && git init")
```

Builders and openers have access to the following variables when they are invoked:

```lua
print("PM_PROJECT_NAME: " .. (PM_PROJECT_NAME or "nil"))
print("PM_PROJECT_PATH: " .. (PM_PROJECT_PATH or "nil"))
print("PM_ALIAS_GROUP: " .. (PM_ALIAS_GROUP or "nil"))
print("PM_PROJECT_TYPE: " .. (PM_PROJECT_TYPE or "nil"))
print("PM_PROJECT_LIB: " .. (PM_PROJECT_LIB or "nil"))
```

An opener could look like this:

```lua
os.execute("cd " .. PM_PROJECT_PATH .. " && code .")
```

There are 3 special cases for openers. You can define an opener for the openers and builders directory, and for the config file. For these openers the only global variables available are

```lua
print("PM_BUILDERS_PATH: " .. (PM_BUILDERS_PATH or "nil"))
print("PM_OPENERS_PATH: " .. (PM_OPENERS_PATH or "nil"))
print("PM_CONFIG_PATH: " .. (PM_CONFIG_PATH or "nil"))
```

This means that a general opener for VS Code would look like:

```lua
-- Determine which path variable is defined
local path = PM_PROJECT_PATH or PM_CONFIG_PATH or PM_OPENERS_PATH or PM_BUILDERS_PATH

if path then
    os.execute("code " .. path)
else
    error("No valid path variable found (PM_PROJECT_PATH, PM_CONFIG_PATH, PM_OPENERS_PATH, or PM_BUILDERS_PATH)")
end
```

Since one of the paths will always be defined. Alternatively, you could check to see if the variable `PM_PROJECT_NAME` is defined to see if the opener is being called for one of the special cases, or if it is being called for a project.

## Installation

Download the binary for your system and add it to your PATH.

## Core Concepts

- **Project**: A directory containing related files for a specific task or application
- **Library**: A collection of projects in a common root directory
- **Alias Group**: A directory containing symbolic links to projects
- **Project Type**: A template defining how projects are created and opened

## Usage

### Getting Help

```bash
donna --help
```

### Creating a New Project

```bash
donna create project my-new-project --project-type rust
```

### Opening a Project

```bash
donna open project my-project
```

To open in terminal mode (prints the path):

```bash
donna open project my-project --terminal
```

### Listing Projects

```bash
donna list projects
```

With additional details:

```bash
donna list projects --libs --types --paths
```

Or show all information:

```bash
donna list projects --all
```

### Library Management

Create a new library:

```bash
donna create lib my-lib /path/to/lib
```

Set as default:

```bash
donna set default-lib my-lib
```

List libraries:

```bash
donna list libraries
```

### Managing Alias Groups

Create an alias group:

```bash
donna create alias-group school /path/to/school/projects
```

List alias groups:

```bash
donna list alias-groups
```

### Project Types

Define a new project type:

```bash
donna create project-type rust --builder path/to/builder.lua --opener path/to/opener.lua
```

List project types:

```bash
donna list project-types
```

### Importing Existing Projects

Import all projects from a directory:

```bash
donna import my-lib /path/to/lib --project-type rust
```

### Managing Configuration

Set builders path:

```bash
donna set builders-path /path/to/builders
```

Set openers path:

```bash
donna set openers-path /path/to/openers
```

### Forgetting Entities

Untrack a library (doesn't delete files):

```bash
donna forget library my-lib
```

Untrack an alias group:

```bash
donna forget alias-group school
```

Untrack a project type:

```bash
donna forget project-type rust
```

### Shell Completion

Generate and install shell completions for a better command-line experience:

**Bash:**
```bash
donna completion bash > ~/.bash_completion.d/donna
```

**Zsh:**
```bash
donna completion zsh > "${fpath[1]}/_donna"
```

**Fish:**
```bash
donna completion fish > ~/.config/fish/completions/donna.fish
```

## Configuration

## Configuration

Donna stores its configuration in `~/.config/donna/config.toml`.

## Design Philosophy

- Never cause breaking changes to projects indirectly
- Never make users reliant on the tool - it's an extension of the OS file system
- Projects can still be managed via Finder, Explorer, or other file managers
- Never delete files - always use system trash when available

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## License

[LICENSE TBD]