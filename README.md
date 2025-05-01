# Project Manager (PM) - Project Proposal

## Overview

The **Project Manager (PM)** is a command-line tool designed to help users manage and organize their projects on a computer. It allows users to create projects from templates (e.g., Python sandbox, MATLAB sandbox, Word documents), define custom configurations, and manage project aliases. The tool will also provide health checks to ensure project aliases and metadata are consistent. The PM will be highly customizable, allowing users to define their own project templates, commands, and workflows through a `.config` file and project-specific scripts.

## Design Philosphies
- Never cause breaking changes to projects inderectly, e.g. changing absolute paths
- Never make the user reliant on the pm, it is just an extension to the OS file system, projects can still easily be manage via finder, etc
- Never delete, always use system trash

---

## Features

### 1. **Project Creation**

-   Users can create projects using predefined or custom templates.
-   Example: `pm create --project project-name alias-folders other-information`
-   Templates include:
    -   **Python Sandbox**: A single-file Python project.
    -   **MATLAB Sandbox**: A MATLAB project.
    -   **Word Project**: A Word document project with a `README` for metadata.
-   Users can define their own templates by creating project-type scripts.

### 2. **Alias Management**

-   Projects can be assigned to one or more alias group folders (e.g., `Archived`, `Python`, `School`).
-   Aliases can be added via:
    -   The `.config` file.
    -   The CLI: `pm create --alias alias-folder-name alias-folder-path`.
-   The PM will maintain a metadata file for each project to track its aliases.

### 3. **Custom Configuration**

-   Users define a `.config` file (a Python file) to customize the behavior of the PM.
-   The `.config` file includes:
    -   Custom scripts for project creation.
    -   Commands for the CLI.
    -   Path to the library directory where all projects are stored.

### 4. **Project-Type Scripts**

-   For each project type (e.g., `word`, `python`), users can define a script that specifies:
    -   How the project should be created.
    -   What `pm open project-name` should do.
    -   Other project-specific commands.
-   The PM will automatically pass arguments to these scripts.
-   A Python package will be provided to simplify script creation and interaction with the PM API.

### 5. **Health Checks**

-   `pm --checkhealth`:
    -   Scans all projects in the library directory.
    -   Ensures all aliases are correctly placed.
    -   Reports inconsistencies.
-   `pm fix`:
    -   Attempts to repair inconsistencies in aliases and metadata.

### 6. **Open Projects**

-   Users can open projects using `pm open project-name`.
-   The behavior of this command is defined in the project-type script.

### 7. **Extensibility**

-   Users can download and share project-type scripts created by others.
-   The PM is designed to be flexible, allowing users to define their own workflows and commands.

---

## Workflow

1. **Setup**:

    - Define a `.config` file to specify the library directory, custom scripts, and commands.
    - Add alias folders using `pm create --alias`.

2. **Create a Project**:

    - Run `pm create --project project-name alias-folders other-information`.
    - The PM will use the appropriate project-type script to create the project and assign it to the specified alias folders.

3. **Manage Projects**:

    - Use `pm open project-name` to open a project.
    - Add or remove aliases using the `.config` file or CLI.

4. **Health Checks**:
    - Regularly run `pm --checkhealth` to ensure aliases and metadata are consistent.
    - Use `pm fix` to repair any issues.

---

## Technical Details

### 1. **File Structure**

-   **Library Directory**: Contains all projects.
-   **Project Metadata**: Each project has a metadata file tracking its aliases and other information.
-   **Project-Type Scripts**: Stored in a designated folder, these scripts define how projects are created and managed.

### 2. **Configuration File**

-   A Python file (`config.py`) where users define:
    -   Library directory path.
    -   Custom scripts for project creation.
    -   CLI commands.
    -   Alias folder paths.

### 3. **Project-Type Scripts**

-   Each script (e.g., `word.py`, `python.py`) defines:
    -   Project creation logic.
    -   Commands like `pm open`.
-   The PM provides a Python package to simplify script creation and argument handling.

### 4. **Health Check Mechanism**

-   The PM scans the library directory and compares actual aliases with metadata.
-   Inconsistencies are reported and can be fixed using `pm fix`.

---

## Example Use Cases

1. **Creating a Python Project**:

    - Run `pm create --project my-python-project Python`.
    - The PM uses the `python.py` script to create a Python sandbox project and assigns it to the `Python` alias folder.

2. **Adding an Alias**:

    - Run `pm create --alias School /path/to/school`.
    - Assign the `School` alias to a project using the `.config` file or CLI.

3. **Health Check**:
    - Run `pm --checkhealth` to ensure all aliases are correctly placed.
    - Use `pm fix` to repair any inconsistencies.

---

## Future Enhancements

1. **GUI Integration**: A graphical interface for users who prefer visual project management.
2. **Cloud Sync**: Sync projects and metadata across devices using cloud storage.
3. **Plugin System**: Allow users to extend functionality with plugins.
4. **Project Sharing**: Enable users to share projects and templates via a centralized repository.

---

## Conclusion

The **Project Manager (PM)** is a powerful and flexible tool for managing projects on a computer. By allowing users to define custom templates, aliases, and workflows, the PM adapts to a wide range of use cases. Its health check and repair features ensure consistency, while its extensibility makes it a valuable tool for both individual users and teams.

[![CI Status](https://github.com/levirogalla/donna/actions/workflows/ci.yml/badge.svg)](https://github.com/levirogalla/donna/actions/workflows/ci.yml)