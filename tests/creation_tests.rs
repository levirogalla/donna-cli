
use donna::{
    create_alias_group, create_lib, create_project, define_project_type, Config, ProjectConfig, XDG
};
mod utils;
use utils::{
    gen_test_alias_groups_path, gen_test_config_home_path, gen_test_data_home_path,
    gen_test_home_path, setup_home,
};

use std::path::{Path, PathBuf};
use rand::prelude::*;

#[test]
fn test_fs_setup() {
    let unique_name = "test_fs_setup";
    let xdg = XDG::new(Some(unique_name));

    let _cleanup = setup_home(unique_name, &xdg);

    assert!(gen_test_config_home_path(unique_name)
        .join("project_manager")
        .exists());
    assert!(gen_test_config_home_path(unique_name)
        .join("project_manager")
        .exists());

    drop(_cleanup);

    assert!(!gen_test_home_path(unique_name).exists());
}

#[test]
fn test_create_project_no_alias_no_type() {
    let unique_name = "test_create_project_no_alias";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);

    create_project("test-proj", None, None, None, false, &xdg);

    assert!(gen_test_data_home_path(unique_name)
        .join("project_manager/projects/test-proj")
        .exists());

    let pm_config = ProjectConfig::load(
        gen_test_data_home_path(unique_name)
            .join("project_manager/projects/test-proj")
            .join(ProjectConfig::PROJECT_ROOT_REL_PATH)
            .to_str()
            .unwrap(),
    );
    assert!(pm_config.is_ok());
    assert!(pm_config.as_ref().unwrap().project_type.is_none());
    assert!(pm_config.as_ref().unwrap().builder.is_none());
    assert!(pm_config.as_ref().unwrap().builder.is_none());
}

#[test]
fn test_create_alias_group() {
    let unique_name = "test_create_alias_group";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);

    let alias_group_path = gen_test_alias_groups_path(unique_name).join("t-alias");
    create_alias_group(
        "test-alias",
        alias_group_path.to_str().unwrap(),
        false,
        &xdg,
    );

    assert!(alias_group_path.exists());
}

#[test]
fn test_create_project_with_alias_and_lib() {
    let unique_name = "test_create_project_with_alias";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);

    create_lib(
        "test-external-lib",
        gen_test_home_path(unique_name)
            .join("lib")
            .to_str()
            .unwrap(),
        false,
        false,
        &xdg,
    );

    let alias_group_path = gen_test_alias_groups_path(unique_name);
    create_alias_group(
        "test-alias1",
        alias_group_path.join("a1").to_str().unwrap(),
        false,
        &xdg,
    );
    create_alias_group(
        "test-alias2",
        alias_group_path.join("a2").to_str().unwrap(),
        false,
        &xdg,
    );

    create_project("test-proj1", None, Some("test-alias1"), None, false, &xdg);
    create_project(
        "test-proj2",
        None,
        Some("test-alias1"),
        Some("test-external-lib"),
        false,
        &xdg,
    );
    create_project(
        "test-proj3",
        None,
        Some("test-alias2"),
        Some("test-external-lib"),
        false,
        &xdg,
    );

    assert!(gen_test_data_home_path(unique_name)
        .join("project_manager/projects/test-proj1")
        .exists());
    assert!(gen_test_home_path(unique_name)
        .join("lib/test-proj3")
        .exists());
    assert!(gen_test_home_path(unique_name)
        .join("lib/test-proj2")
        .exists());

    assert!(alias_group_path
        .join("a1/test-proj1/.pm/project.toml")
        .exists());
    assert!(alias_group_path
        .join("a1/test-proj2/.pm/project.toml")
        .exists());
    assert!(alias_group_path
        .join("a2/test-proj3/.pm/project.toml")
        .exists());
}

#[test]
fn test_create_project_with_type() {
    let unique_name = "test_create_project_with_type";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);

    define_project_type("test-project-type", None, None, None, false, &xdg);

    create_project(
        "test-proj",
        Some("test-project-type"),
        None,
        None,
        false,
        &xdg,
    );

    assert!(gen_test_data_home_path(unique_name)
        .join("project_manager/projects/test-proj")
        .exists());

    let pm_config = ProjectConfig::load(
        gen_test_data_home_path(unique_name)
            .join("project_manager/projects/test-proj")
            .join(ProjectConfig::PROJECT_ROOT_REL_PATH)
            .to_str()
            .unwrap(),
    );
    assert!(pm_config.is_ok());
    assert_eq!(
        pm_config.as_ref().unwrap().project_type,
        Some("test-project-type".to_string())
    );
    assert!(pm_config.as_ref().unwrap().builder.is_none());
    assert!(pm_config.as_ref().unwrap().builder.is_none());
}

#[test]
fn test_create_projects_with_libs() {
    let unique_name = "test_create_project_with_lib";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);

    create_lib(
        "test-default-lib",
        gen_test_home_path(unique_name)
            .join("lib-d")
            .to_str()
            .unwrap(),
        true,
        false,
        &xdg,
    );

    create_lib(
        "test-non-default-lib1",
        gen_test_home_path(unique_name)
            .join("lib1")
            .to_str()
            .unwrap(),
        false,
        false,
        &xdg,
    );

    create_lib(
        "test-non-default-lib2",
        gen_test_home_path(unique_name)
            .join("lib2")
            .to_str()
            .unwrap(),
        false,
        false,
        &xdg,
    );

    create_project("default-proj", None, None, None, false, &xdg);
    create_project(
        "lib1-proj",
        None,
        None,
        Some("test-non-default-lib1"),
        false,
        &xdg,
    );
    create_project(
        "lib2-proj",
        None,
        None,
        Some("test-non-default-lib2"),
        false,
        &xdg,
    );

    create_lib(
        "test-default-lib-override",
        gen_test_home_path(unique_name)
            .join("lib-do")
            .to_str()
            .unwrap(),
        true,
        false,
        &xdg,
    );

    create_project("default-proj-2", None, None, None, false, &xdg);
    create_project(
        "old-default-proj",
        None,
        None,
        Some("test-default-lib"),
        false,
        &xdg,
    );

    assert!(gen_test_home_path(unique_name)
        .join("lib-d/default-proj")
        .exists());
    assert!(gen_test_home_path(unique_name)
        .join("lib-d/old-default-proj")
        .exists());
    assert!(gen_test_home_path(unique_name)
        .join("lib1/lib1-proj")
        .exists());
    assert!(gen_test_home_path(unique_name)
        .join("lib2/lib2-proj")
        .exists());
    assert!(gen_test_home_path(unique_name)
        .join("lib-do/default-proj-2")
        .exists());
}

#[test]
fn test_create_many_projects_with_type_and_alias_and_lib() {
    let unique_name = "test_create_many_projects_with_type_and_alias_and_lib";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);

    let home_path = gen_test_home_path(unique_name);

    create_lib(
        "lib1",
        home_path.join("lib1").to_str().unwrap(),
        false,
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
        "default",
        home_path.join("default").to_str().unwrap(),
        true,
        false,
        &xdg,
    );

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

    define_project_type(
        "type1",
        Some(vec!["alias1".to_string(), "alias2".to_string()]),
        None,
        None,
        false,
        &xdg,
    );
    define_project_type("type2", Some(vec!["alias3".to_string()]), None, None, false, &xdg);

    struct Project {
        name: String,
        alias_group: Option<String>,
        lib: Option<String>,
        project_type: Option<String>,
    }

    let mut created_projects: Vec<Project> = Vec::new();

    let alias_groups = [Some("alias1"), Some("alias2"), Some("alias3"), None];
    let libs = [Some("lib1"), Some("lib2"), None];
    let project_types = [Some("type1"), Some("type2"), None];

    let mut rng = rand::rng();

    for i in 0..1_000 {
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

        create_project(&project_name, project_type, alias_group, lib, false, &xdg);

        created_projects.push(Project {
            name: project_name,
            alias_group: alias_group.map(|s| s.to_string()),
            lib: lib.map(|s| s.to_string()),
            project_type: project_type.map(|s| s.to_string()),
        });
    }

    for project in created_projects {
        let project_path = home_path
            .join(project.lib.as_deref().unwrap_or("default"))
            .join(&project.name);
        assert!(project_path.exists(), "Project {} not found", project.name);

        let pm_config = ProjectConfig::load(
            project_path
                .join(ProjectConfig::PROJECT_ROOT_REL_PATH)
                .to_str()
                .unwrap(),
        );
        assert!(pm_config.is_ok());
        assert_eq!(
            pm_config.as_ref().unwrap().project_type,
            project.project_type
        );

        match (&project.project_type, &project.alias_group) {
            (Some(ref project_type), None) if project_type == "type1" => {
                assert!(home_path.join("alias1").join(&project.name).exists());
                assert!(home_path.join("alias2").join(&project.name).exists());
                assert!(!home_path.join("alias3").join(&project.name).exists());
            }
            (Some(ref project_type), None) if project_type == "type2" => {
                assert!(!home_path.join("alias1").join(&project.name).exists());
                assert!(!home_path.join("alias2").join(&project.name).exists());
                assert!(home_path.join("alias3").join(&project.name).exists());
            }
            (None, Some(ref alias_group)) if alias_group == "alias1" => {
                assert!(home_path
                    .join("alias1")
                    .join(&project.name)
                    .join(".pm/project.toml")
                    .exists());
                assert!(!home_path
                    .join("alias2")
                    .join(&project.name)
                    .join(".pm/project.toml")
                    .exists());
                assert!(!home_path
                    .join("alias3")
                    .join(&project.name)
                    .join(".pm/project.toml")
                    .exists());
            }
            (None, Some(ref alias_group)) if alias_group == "alias2" => {
                assert!(!home_path
                    .join("alias1")
                    .join(&project.name)
                    .join(".pm/project.toml")
                    .exists());
                assert!(home_path
                    .join("alias2")
                    .join(&project.name)
                    .join(".pm/project.toml")
                    .exists());
                assert!(!home_path
                    .join("alias3")
                    .join(&project.name)
                    .join(".pm/project.toml")
                    .exists());
            }
            (None, Some(ref alias_group)) if alias_group == "alias3" => {
                assert!(!home_path
                    .join("alias1")
                    .join(&project.name)
                    .join(".pm/project.toml")
                    .exists());
                assert!(!home_path
                    .join("alias2")
                    .join(&project.name)
                    .join(".pm/project.toml")
                    .exists());
                assert!(home_path
                    .join("alias3")
                    .join(&project.name)
                    .join(".pm/project.toml")
                    .exists());
            }
            (None, None) => {
                assert!(!home_path.join("alias1").join(&project.name).exists());
                assert!(!home_path.join("alias2").join(&project.name).exists());
                assert!(!home_path.join("alias3").join(&project.name).exists());
            }
            _ => {
                println!(
                    "Case not tested: project_type: {:?}, alias_group: {:?}",
                    project.project_type, project.alias_group
                );
            }
        }
    }
}

#[test]
#[ignore]
fn test_incorrect_usage() {
    todo!();
    // test incorrect usage of the API
    // like creating a project with an alias that doesn't exist
    // or creating a project with a type that doesn't exist
    // or creating a project with a lib that doesn't exist
}
// handle incorrect usage, like making two alias groups with the same name or libraries with the same name

#[test]
fn test_relative_paths_are_handled_properly() {
    let unique_name = "test_relative_paths_are_handled_properly";
    let xdg = XDG::new(Some(unique_name));
    let mut _cleanup = setup_home(unique_name, &xdg);

    let cwd = std::env::current_dir().unwrap();

    gen_test_home_path(unique_name);
    let test_home = Path::new("./tests/test_home_dirs").join(unique_name);
    // make sure we are running tests from root dir
    assert!(cwd.read_dir().unwrap().filter(|f| ["tests", "Cargo.toml", "src"].contains(&f.as_ref().unwrap().file_name().to_str().unwrap())).count() == 3);

    create_lib("lib", test_home.join("lib").to_str().unwrap(), true, false, &xdg);
    create_alias_group("group", test_home.join("group").to_str().unwrap(), false, &xdg);

    let config = Config::load(None, &xdg).unwrap();
    assert!(PathBuf::from(config.get_alias_group("group").unwrap().path.as_str()).is_absolute());
    assert!(PathBuf::from(config.get_lib_path(Some("lib")).unwrap()).is_absolute());

}