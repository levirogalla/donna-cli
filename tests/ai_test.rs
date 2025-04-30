mod utils;

use donna::{
    create_alias_group, create_lib, create_project, define_project_type, update_alias_group,
    untrack_alias_group, delete_alias_group, open_project, Config, ProjectConfig, XDG,
};

use rand::prelude::*;
use std::{fs, io::Write, path::Path};
use utils::{
    gen_test_data_home_path,
    gen_test_home_path, setup_home,
};

// Comprehensive API functionality tests

#[test]
fn test_api_operations_sequence() {
    // This test verifies the entire lifecycle of a project including:
    // 1. Creating libraries, alias groups, and project types
    // 2. Creating projects with various combinations
    // 3. Opening projects
    // 4. Updating and deleting alias groups
    
    let unique_name = "test_api_operations_sequence";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    let home_path = gen_test_home_path(unique_name);
    
    // First, set up our environment
    // Create libraries
    create_lib(
        "main-lib",
        home_path.join("main-lib").to_str().unwrap(),
        true,
        false,
        &xdg,
    );
    
    create_lib(
        "secondary-lib",
        home_path.join("sec-lib").to_str().unwrap(),
        false,
        false,
        &xdg,
    );
    
    // Create alias groups
    create_alias_group(
        "work",
        home_path.join("work-projects").to_str().unwrap(),
        false,
        &xdg,
    );
    
    create_alias_group(
        "personal",
        home_path.join("personal-projects").to_str().unwrap(),
        false,
        &xdg,
    );
    
    // Create project types with different configurations
    define_project_type(
        "rust-project",
        Some(vec!["work".to_string()]),
        None,
        None,
        false,
        &xdg,
    );
    
    // Create project type with a builder script
    let lua_script_path = home_path.join("builder.lua");
    let mut f = fs::File::create(&lua_script_path).unwrap();
    f.write(b"
        local command = string.format(\"cd %s && echo 'Project created: %s' > created.txt\",
                        PM_PROJECT_PATH, PM_PROJECT_NAME)
        os.execute(command)
    ").unwrap();
    
    define_project_type(
        "scripted-project",
        Some(vec!["personal".to_string()]),
        Some(lua_script_path.to_str().unwrap()),
        None,
        false,
        &xdg,
    );
    
    // Create project with a specific type and lib
    create_project(
        "rust-work-project",
        Some("rust-project"),
        None,  // Using the default from project type
        Some("main-lib"),
        false,
        &xdg,
    );
    
    // Verify project was created in the correct library
    assert!(home_path
        .join("main-lib/rust-work-project")
        .exists());
    
    // Verify alias was created as defined in the project type
    assert!(home_path
        .join("work-projects/rust-work-project")
        .exists());
    
    // Create project with a scripted project type that runs a builder script
    create_project(
        "script-personal-project",
        Some("scripted-project"),
        None, // Using default from project type
        Some("secondary-lib"),
        false,
        &xdg,
    );
    
    // Verify builder script ran by checking for created.txt
    assert!(home_path
        .join("sec-lib/script-personal-project/created.txt")
        .exists());
        
    // Read and verify the content of created.txt
    let created_content = fs::read_to_string(
        home_path.join("sec-lib/script-personal-project/created.txt")
    ).unwrap();
    assert!(created_content.contains("Project created: script-personal-project"));
    
    // Create project with explicit alias group
    create_project(
        "explicit-alias-project",
        None,
        Some("work"),
        None, // Using default lib 
        false,
        &xdg,
    );
    
    // Verify project was created in default lib
    assert!(home_path
        .join("main-lib/explicit-alias-project")
        .exists());
    
    // Verify alias was created
    assert!(home_path
        .join("work-projects/explicit-alias-project")
        .exists());
    
    // Test updating an alias group
    update_alias_group(
        "work",
        Some("business"),
        Some(home_path.join("business-projects").to_str().unwrap()),
        &xdg,
    );
    
    // Verify the alias path was updated
    let config = Config::load(None, &xdg).unwrap();
    assert_eq!(
        config.get_alias_group("business").unwrap().path,
        home_path.join("business-projects").to_str().unwrap()
    );
    
    // Create a project with multiple alias groups to test untracking
    create_project(
        "multi-alias-project",
        None,
        Some("business"), // renamed from "work"
        None,
        false,
        &xdg,
    );
    
    // Manually add another alias group to the project
    let project_path = home_path.join("main-lib/multi-alias-project");
    let project_config_path = project_path.join(ProjectConfig::PROJECT_ROOT_REL_PATH);
    let mut project_config = ProjectConfig::load(project_config_path.to_str().unwrap()).unwrap();
    
    // Add personal alias group to tracked groups
    project_config.tracked_alias_groups.as_mut().unwrap().push("personal".to_string());
    project_config.save(project_config_path.to_str().unwrap()).unwrap();
    
    // Create the symlink manually
    std::os::unix::fs::symlink(
        &project_path,
        home_path.join("personal-projects/multi-alias-project")
    ).unwrap();
    
    // Test untracking an alias group
    untrack_alias_group("personal", &xdg);
    
    // Verify the alias is no longer tracked in the config
    let project_config = ProjectConfig::load(project_config_path.to_str().unwrap()).unwrap();
    assert!(!project_config.tracked_alias_groups.unwrap().contains(&"personal".to_string()));
    
    // Test deleting an alias group
    delete_alias_group("business", &xdg);
    
    // Verify the alias group no longer exists in config
    let config = Config::load(None, &xdg).unwrap();
    assert!(config.get_alias_group("business").is_none());
}

#[test]
fn test_project_type_precedence() {
    // Test that project type settings take precedence over explicitly specified settings
    
    let unique_name = "test_project_type_precedence";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    let home_path = gen_test_home_path(unique_name);
    
    // Create libraries
    create_lib(
        "default-lib",
        home_path.join("default").to_str().unwrap(),
        true,
        false,
        &xdg,
    );
    
    // Create alias groups
    create_alias_group(
        "group1",
        home_path.join("group1").to_str().unwrap(),
        false,
        &xdg,
    );
    
    create_alias_group(
        "group2",
        home_path.join("group2").to_str().unwrap(),
        false,
        &xdg,
    );
    
    // Create project type with specific alias groups
    define_project_type(
        "type-with-aliases",
        Some(vec!["group1".to_string(), "group2".to_string()]),
        None,
        None,
        false,
        &xdg,
    );
    
    // Create project with the project type but also specifying a different alias group
    create_project(
        "precedence-project",
        Some("type-with-aliases"),
        Some("group2"), // Only specify group2 explicitly
        None,
        false,
        &xdg,
    );
    
    // Verify that both alias groups from the project type were created
    assert!(home_path.join("group1/precedence-project").exists());
    assert!(home_path.join("group2/precedence-project").exists());
    
    // Verify the project config has both alias groups
    let project_config = ProjectConfig::load(
        home_path.join("default/precedence-project")
            .join(ProjectConfig::PROJECT_ROOT_REL_PATH)
            .to_str().unwrap()
    ).unwrap();
    
    let tracked_groups = project_config.tracked_alias_groups.unwrap();
    assert!(tracked_groups.contains(&"group1".to_string()));
    assert!(tracked_groups.contains(&"group2".to_string()));
}

#[test]
fn test_project_config_consistency() {
    // Test that project configuration is saved and loaded correctly
    let unique_name = "test_project_config_consistency";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    // Create project with specific project type and builder/opener
    let home_path = gen_test_home_path(unique_name);
    
    // Create a builder script
    let builder_path = home_path.join("custom_builder.lua");
    let mut f = fs::File::create(&builder_path).unwrap();
    f.write(b"print('Custom builder')").unwrap();
    
    // Create an opener script
    let opener_path = home_path.join("custom_opener.lua");
    let mut f = fs::File::create(&opener_path).unwrap();
    f.write(b"print('Custom opener')").unwrap();
    
    // Define project type with custom scripts
    define_project_type(
        "custom-scripted-type",
        None,
        Some(builder_path.to_str().unwrap()),
        Some(opener_path.to_str().unwrap()),
        false,
        &xdg,
    );
    
    // Create project with this type
    create_project(
        "config-test-project",
        Some("custom-scripted-type"),
        None,
        None,
        false,
        &xdg,
    );
    
    // Load the project config
    let project_path = gen_test_data_home_path(unique_name)
        .join("project_manager/projects/config-test-project");
    let project_config_path = project_path.join(ProjectConfig::PROJECT_ROOT_REL_PATH);
    
    let project_config = ProjectConfig::load(project_config_path.to_str().unwrap()).unwrap();
    
    // Verify config values
    assert_eq!(project_config.project_type.as_deref(), Some("custom-scripted-type"));
    assert_eq!(project_config.builder.as_deref(), Some(builder_path.to_str().unwrap()));
    assert_eq!(project_config.opener.as_deref(), Some(opener_path.to_str().unwrap()));
}

// Error handling tests

#[test]
#[should_panic(expected = "Project type already exists")]
fn test_duplicate_project_type() {
    let unique_name = "test_duplicate_project_type";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    // Define a project type
    define_project_type("duplicate-type", None, None, None, false, &xdg);
    
    // Try to define it again without redefine flag
    define_project_type("duplicate-type", None, None, None, false, &xdg);
}

#[test]
fn test_redefine_project_type() {
    let unique_name = "test_redefine_project_type";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    // Define a project type
    define_project_type("redefine-type", None, None, None, false, &xdg);
    
    // Define it again with redefine flag
    define_project_type(
        "redefine-type", 
        Some(vec!["new-default".to_string()]), 
        None, 
        None, 
        true, 
        &xdg
    );
    
    // Verify it was redefined
    let config = Config::load(None, &xdg).unwrap();
    let project_type = config.get_project_type("redefine-type".to_string()).unwrap();
    assert_eq!(
        project_type.default_alias_groups.as_ref().unwrap()[0],
        "new-default"
    );
}

#[test]
#[should_panic(expected = "Project type does not exist")]
fn test_redefine_nonexistent_project_type() {
    let unique_name = "test_redefine_nonexistent";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    // Try to redefine a non-existent project type
    define_project_type(
        "nonexistent-type", 
        None, 
        None, 
        None, 
        true, 
        &xdg
    );
}

#[test]
fn test_create_and_open_project() {
    // Comprehensive test of project creation and opening
    let unique_name = "test_create_and_open";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    let home_path = gen_test_home_path(unique_name);
    
    // Create an opener script to verify it runs on open
    let opener_path = home_path.join("open_verify.lua");
    let mut f = fs::File::create(&opener_path).unwrap();
    f.write(b"
        local command = string.format(\"cd %s && echo 'Opened: %s in %s' > opened.txt\",
                        PM_PROJECT_PATH, PM_PROJECT_NAME, PM_PROJECT_LIB)
        os.execute(command)
    ").unwrap();
    
    // Define a project type with this opener
    define_project_type(
        "openable-type",
        None,
        None,
        Some(opener_path.to_str().unwrap()),
        false,
        &xdg,
    );
    
    // Create a project with this type
    create_project(
        "openable-project",
        Some("openable-type"),
        None,
        None,
        false,
        &xdg,
    );
    
    // Open the project
    open_project("openable-project", None, &xdg);
    
    // Check if the opener script ran by looking for opened.txt
    let project_path = gen_test_data_home_path(unique_name)
        .join("project_manager/projects/openable-project");
    
    assert!(project_path.join("opened.txt").exists());
    
    // Verify the content of opened.txt
    let opened_content = fs::read_to_string(project_path.join("opened.txt")).unwrap();
    assert!(opened_content.contains("Opened: openable-project"));
}

#[test]
fn test_many_projects_with_random_configurations() {
    // Create many projects with random configurations to test system robustness
    let unique_name = "test_many_random_projects";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    let home_path = gen_test_home_path(unique_name);
    
    // Create several libraries
    create_lib(
        "lib1",
        home_path.join("lib1").to_str().unwrap(),
        true, // default
        false,
        &xdg,
    );
    
    create_lib(
        "lib2",
        home_path.join("lib2").to_str().unwrap(),
        false,
        false,
        &xdg,
    );
    
    create_lib(
        "lib3",
        home_path.join("lib3").to_str().unwrap(),
        false,
        false,
        &xdg,
    );
    
    // Create several alias groups
    create_alias_group(
        "alias1",
        home_path.join("alias1").to_str().unwrap(),
        false,
        &xdg,
    );
    
    create_alias_group(
        "alias2",
        home_path.join("alias2").to_str().unwrap(),
        false,
        &xdg,
    );
    
    create_alias_group(
        "alias3",
        home_path.join("alias3").to_str().unwrap(),
        false,
        &xdg,
    );
    
    // Create several project types with different configurations
    define_project_type(
        "type1",
        Some(vec!["alias1".to_string(), "alias2".to_string()]),
        None,
        None,
        false,
        &xdg,
    );
    
    define_project_type(
        "type2", 
        Some(vec!["alias3".to_string()]), 
        None,
        None,
        false,
        &xdg
    );
    
    define_project_type(
        "type3",
        None,
        None,
        None,
        false,
        &xdg
    );
    
    // Create a struct to track created projects and their expected configurations
    struct Project {
        name: String,
        project_type: Option<String>,
        alias_group: Option<String>,
        lib: Option<String>,
    }
    
    let mut created_projects: Vec<Project> = Vec::new();
    
    let alias_groups = [Some("alias1"), Some("alias2"), Some("alias3"), None];
    let libs = [Some("lib1"), Some("lib2"), Some("lib3"), None];
    let project_types = [Some("type1"), Some("type2"), Some("type3"), None];
    
    let mut rng = rand::thread_rng();
    
    // Create 50 projects with random combinations
    for i in 0..50 {
        let lib = *libs.choose(&mut rng).unwrap();
        let alias_group = *alias_groups.choose(&mut rng).unwrap();
        let project_type = *project_types.choose(&mut rng).unwrap();
        
        let project_name = format!(
            "{}-{}-{}-{}",
            project_type.unwrap_or("default"),
            alias_group.unwrap_or("default"),
            lib.unwrap_or("default"),
            i
        );
        
        create_project(
            &project_name,
            project_type,
            alias_group,
            lib,
            false,
            &xdg,
        );
        
        created_projects.push(Project {
            name: project_name,
            project_type: project_type.map(|s| s.to_string()),
            alias_group: alias_group.map(|s| s.to_string()),
            lib: lib.map(|s| s.to_string()),
        });
    }
    
    // Verify all projects were created with correct configurations
    for project in created_projects {
        // Determine expected path based on library
        let project_path = if let Some(lib) = &project.lib {
            home_path.join(lib).join(&project.name)
        } else {
            home_path.join("lib1").join(&project.name) // Default lib
        };
        
        // Verify project exists
        assert!(
            project_path.exists(),
            "Project {} should exist at {}",
            project.name,
            project_path.display()
        );
        
        // Load project config
        let pm_config = ProjectConfig::load(
            project_path
                .join(ProjectConfig::PROJECT_ROOT_REL_PATH)
                .to_str()
                .unwrap(),
        ).unwrap();
        
        // Verify project type
        assert_eq!(
            pm_config.project_type, 
            project.project_type,
            "Project {} should have type {:?} but has {:?}",
            project.name,
            project.project_type,
            pm_config.project_type
        );
        
        // Verify aliases were created according to project type and explicit alias
        if let Some(ref project_type) = project.project_type {
            match project_type.as_str() {
                "type1" => {
                    // Should have alias1 and alias2
                    assert!(
                        home_path.join("alias1").join(&project.name).exists(),
                        "Project {} should have alias1",
                        project.name
                    );
                    assert!(
                        home_path.join("alias2").join(&project.name).exists(),
                        "Project {} should have alias2",
                        project.name
                    );
                },
                "type2" => {
                    // Should have alias3
                    assert!(
                        home_path.join("alias3").join(&project.name).exists(),
                        "Project {} should have alias3",
                        project.name
                    );
                },
                _ => {}
            }
        }
        
        // Check explicit alias group
        if let Some(alias) = &project.alias_group {
            assert!(
                home_path.join(alias).join(&project.name).exists(),
                "Project {} should have alias {}",
                project.name,
                alias
            );
            
            // Verify it's tracked in the config
            assert!(
                pm_config.tracked_alias_groups.unwrap().contains(&alias.to_string()),
                "Project {} should track alias {}",
                project.name,
                alias
            );
        }
    }
}

#[test]
fn test_relative_path_handling() {
    // Test that relative paths are properly converted to absolute paths
    let unique_name = "test_relative_path_handling";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    // Use relative paths for creating alias groups and libraries
    let test_home = Path::new("./tests/test_home_dirs").join(unique_name);
    
    // Create a library with relative path
    create_lib(
        "rel-lib",
        test_home.join("rel-lib").to_str().unwrap(),
        true,
        false,
        &xdg,
    );
    
    // Create an alias group with relative path
    create_alias_group(
        "rel-alias",
        test_home.join("rel-alias").to_str().unwrap(),
        false,
        &xdg,
    );
    
    // Load the config
    let config = Config::load(None, &xdg).unwrap();
    
    // Verify paths were converted to absolute
    assert!(Path::new(config.get_alias_group("rel-alias").unwrap().path.as_str()).is_absolute());
    assert!(Path::new(config.get_lib_path(Some("rel-lib")).unwrap()).is_absolute());
}

#[test]
fn test_handoff_existing_project() {
    // Test handing off an existing project to the project manager
    let unique_name = "test_handoff_project";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    let home_path = gen_test_home_path(unique_name);
    
    // Create a library
    create_lib(
        "handoff-lib",
        home_path.join("handoff-lib").to_str().unwrap(),
        true,
        false,
        &xdg,
    );
    
    // Create an alias group
    create_alias_group(
        "handoff-alias",
        home_path.join("handoff-alias").to_str().unwrap(),
        false,
        &xdg,
    );
    
    // Create a project directory manually
    let project_path = home_path.join("handoff-lib/existing-project");
    fs::create_dir_all(&project_path).unwrap();
    
    // Create some content in the project
    fs::write(
        project_path.join("test-file.txt"),
        "This is a pre-existing file"
    ).unwrap();
    
    // Hand off the existing project to PM
    create_project(
        "existing-project",
        None,
        Some("handoff-alias"),
        Some("handoff-lib"),
        true, // already exists
        &xdg,
    );
    
    // Verify the project was registered
    let project_config_path = project_path.join(ProjectConfig::PROJECT_ROOT_REL_PATH);
    assert!(project_config_path.exists());
    
    // Verify the alias was created
    assert!(home_path.join("handoff-alias/existing-project").exists());
    
    // Verify the pre-existing content was preserved
    assert!(project_path.join("test-file.txt").exists());
    let content = fs::read_to_string(project_path.join("test-file.txt")).unwrap();
    assert_eq!(content, "This is a pre-existing file");
}

// Config-related tests

#[test]
fn test_config_operations() {
    // Test various config operations including loading, saving, and modifying
    let unique_name = "test_config_operations";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    // Load the initial config
    let config = Config::load(None, &xdg).unwrap();
    
    // Add an alias group
    let home_path = gen_test_home_path(unique_name);
    let alias_path = home_path.join("new-alias");
    fs::create_dir_all(&alias_path).unwrap();
    
    // Use the API to add components
    create_alias_group(
        "config-alias",
        alias_path.to_str().unwrap(),
        true, // already exists
        &xdg,
    );
    
    create_lib(
        "config-lib",
        home_path.join("new-lib").to_str().unwrap(),
        false,
        false,
        &xdg,
    );
    
    // Reload the config and verify changes
    let config = Config::load(None, &xdg).unwrap();
    
    // Verify alias group was added
    let alias = config.get_alias_group("config-alias").unwrap();
    assert_eq!(alias.path, alias_path.to_str().unwrap());
    
    // Verify library was added
    let lib_path = config.get_lib_path(Some("config-lib")).unwrap();
    assert_eq!(lib_path, home_path.join("new-lib").to_str().unwrap());
}

// Let's add tests that exercise edge cases and error handling

#[test]
#[should_panic(expected = "Could not find alias group")]
fn test_nonexistent_alias_error() {
    // Test error handling when referencing a nonexistent alias group
    let unique_name = "test_nonexistent_alias";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    create_project(
        "error-project",
        None,
        Some("nonexistent-alias"), // This alias doesn't exist
        None,
        false,
        &xdg,
    );
}

#[test]
#[should_panic(expected = "Could not find lib path")]
fn test_nonexistent_lib_error() {
    // Test error handling when referencing a nonexistent library
    let unique_name = "test_nonexistent_lib";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    create_project(
        "error-project",
        None,
        None,
        Some("nonexistent-lib"), // This library doesn't exist
        false,
        &xdg,
    );
}

#[test]
#[should_panic(expected = "Could not find project type")]
fn test_nonexistent_project_type_error() {
    // Test error handling when referencing a nonexistent project type
    let unique_name = "test_nonexistent_project_type";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);
    
    create_lib(
        "error-lib",
        gen_test_home_path(unique_name).join("error-lib").to_str().unwrap(),
        true,
        false,
        &xdg,
    );
    
    create_project(
        "error-project",
        Some("nonexistent-type"), // This project type doesn't exist
        None,
        None,
        false,
        &xdg,
    );
}