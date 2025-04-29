mod utils;
use utils::{
    gen_test_home_path, setup_home,
};

use cli_project_manager::{
    create_alias_group, create_lib, create_project, define_project_type, open_project,
    ProjectConfig, XDG,
};

use std::{fs, io::Write};

#[test]
fn test_opener_with_only_defaults() {
    let unique_name = "test_opener_with_only_defaults";
    let xdg = XDG::new(Some(unique_name));
    let mut _cleanup = setup_home(unique_name, &xdg);
    // _cleanup.drop = false;

    let home_dir_path = gen_test_home_path(unique_name);
    let mut f = fs::File::create(home_dir_path.join("opener.lua")).unwrap();
    f.write(b"
    local command = string.format(\"cd %s && echo \'%s, %s, %s, %s\' > proof.txt\",
                    PM_PROJECT_PATH, PM_PROJECT_NAME, PM_PROJECT_PATH, PM_PROJECT_TYPE, PM_PROJECT_LIB
                  )
    print(PM_PROJECT_PATH)
    os.execute(command)").unwrap();

    define_project_type(
        "test-type",
        None,
        None,
        Some(
            gen_test_home_path(unique_name)
                .join("opener.lua")
                .to_str()
                .unwrap(),
        ),
        false,
        &xdg,
    );
    create_project("test-proj", Some("test-type"), None, None, false, &xdg);

    open_project("test-proj", None, &xdg);

    let proof_path =
        home_dir_path.join(".local/share/project_manager/projects/test-proj/proof.txt");
    assert!(proof_path.exists());

    let proof_contents = fs::read_to_string(&proof_path).unwrap();
    assert_eq!(
        proof_contents,
        format!(
            "{}, {}, {}, {}\n",
            "test-proj",
            home_dir_path
                .join(".local/share/project_manager/projects/test-proj")
                .to_str()
                .unwrap(),
            "test-type",
            "nil"
        )
    );

    let project_config = ProjectConfig::load(
        home_dir_path
            .join(".local/share/project_manager/projects/test-proj")
            .join(ProjectConfig::PROJECT_ROOT_REL_PATH)
            .to_str()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(project_config.project_type.as_deref(), Some("test-type"));
    assert_eq!(
        project_config.opener.as_deref(),
        home_dir_path.join("opener.lua").to_str()
    );
    assert_eq!(project_config.builder.as_deref(), None);
}

#[test]
fn test_opener() {
    let unique_name = "test_opener";
    let xdg = XDG::new(Some(unique_name));
    let _cleanup = setup_home(unique_name, &xdg);

    let home_dir_path = gen_test_home_path(unique_name);
    let mut f = fs::File::create(home_dir_path.join("opener.lua")).unwrap();
    f.write(b"
      local command = string.format(\"cd %s && echo \'%s, %s, %s, %s\' > proof.txt\",
                      PM_PROJECT_PATH, PM_PROJECT_NAME, PM_PROJECT_PATH, PM_PROJECT_TYPE, PM_PROJECT_LIB
                    )

      os.execute(command)").unwrap();

    define_project_type(
        "test-type",
        None,
        None,
        Some(
            gen_test_home_path(unique_name)
                .join("opener.lua")
                .to_str()
                .unwrap(),
        ),
        false,
        &xdg,
    );
    create_alias_group(
        "test-group",
        home_dir_path.join("group").to_str().unwrap(),
        false,
        &xdg,
    );
    create_lib(
        "test-lib",
        home_dir_path.join("test-lib").to_str().unwrap(),
        false,
        false,
        &xdg,
    );

    create_project(
        "test-proj",
        Some("test-type"),
        Some("test-group"),
        Some("test-lib"),
        false, 
        &xdg,
    );
    open_project("test-proj", Some("test-lib"), &xdg);

    let proof_path = home_dir_path.join("test-lib/test-proj/proof.txt");
    assert!(proof_path.exists());

    let proof_contents = fs::read_to_string(&proof_path).unwrap();
    assert_eq!(
        proof_contents,
        format!(
            "{}, {}, {}, {}\n",
            "test-proj",
            home_dir_path.join("test-lib/test-proj").to_str().unwrap(),
            "test-type",
            "test-lib"
        )
    );

    let project_config = ProjectConfig::load(
        home_dir_path
            .join("test-lib/test-proj")
            .join(ProjectConfig::PROJECT_ROOT_REL_PATH)
            .to_str()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(project_config.project_type.as_deref(), Some("test-type"));
    assert_eq!(
        project_config.opener.as_deref(),
        home_dir_path.join("opener.lua").to_str()
    );
    assert_eq!(project_config.builder.as_deref(), None);
}

#[test]
fn test_builder_with_only_defaults() {
    let unique_name = "test_builder_with_only_defaults";
    let xdg = XDG::new(Some(unique_name));
    let mut _cleanup = setup_home(unique_name, &xdg);
    // _cleanup.drop = false;

    let home_dir_path = gen_test_home_path(unique_name);
    let mut f = fs::File::create(home_dir_path.join("builder.lua")).unwrap();
    f.write(b"
    local command = string.format(\"cd %s && echo \'%s, %s, %s, %s\' > proof.txt\",
                    PM_PROJECT_PATH, PM_PROJECT_NAME, PM_PROJECT_PATH, PM_PROJECT_TYPE, PM_PROJECT_LIB
                  )
    print(PM_PROJECT_PATH)
    os.execute(command)").unwrap();

    define_project_type(
        "test-type",
        None,
        None,
        Some(
            gen_test_home_path(unique_name)
                .join("builder.lua")
                .to_str()
                .unwrap(),
        ),
        false,
        &xdg,
    );
    create_project("test-proj", Some("test-type"), None, None, false, &xdg);

    open_project("test-proj", None, &xdg);

    let proof_path =
        home_dir_path.join(".local/share/project_manager/projects/test-proj/proof.txt");
    assert!(proof_path.exists());

    let proof_contents = fs::read_to_string(&proof_path).unwrap();
    assert_eq!(
        proof_contents,
        format!(
            "{}, {}, {}, {}\n",
            "test-proj",
            home_dir_path
                .join(".local/share/project_manager/projects/test-proj")
                .to_str()
                .unwrap(),
            "test-type",
            "nil"
        )
    );

    let project_config = ProjectConfig::load(
        home_dir_path
            .join(".local/share/project_manager/projects/test-proj")
            .join(ProjectConfig::PROJECT_ROOT_REL_PATH)
            .to_str()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(project_config.project_type.as_deref(), Some("test-type"));
    assert_eq!(
        project_config.opener.as_deref(),
        home_dir_path.join("builder.lua").to_str()
    );
    assert_eq!(project_config.builder.as_deref(), None);
}

#[test]
fn test_builder() {
    let unique_name = "test_builder";
    let xdg = XDG::new(Some(unique_name));
    let mut _cleanup = setup_home(unique_name, &xdg);

    let home_dir_path = gen_test_home_path(unique_name);
    let mut f = fs::File::create(home_dir_path.join("builder.lua")).unwrap();
    f.write(b"
      local command = string.format(\"cd %s && echo \'%s, %s, %s, %s\' > proof.txt\",
                      PM_PROJECT_PATH, PM_PROJECT_NAME, PM_PROJECT_PATH, PM_PROJECT_TYPE, PM_PROJECT_LIB
                    )

      os.execute(command)").unwrap();

    define_project_type(
        "test-type",
        None,
        Some(
            gen_test_home_path(unique_name)
                .join("builder.lua")
                .to_str()
                .unwrap(),
        ),
        None,
        false,
        &xdg,
    );
    create_alias_group(
        "test-group",
        home_dir_path.join("group").to_str().unwrap(),
        false,
        &xdg,
    );
    create_lib(
        "test-lib",
        home_dir_path.join("test-lib").to_str().unwrap(),
        false,
        false,
        &xdg,
    );

    create_project(
        "test-proj",
        Some("test-type"),
        Some("test-group"),
        Some("test-lib"),
        false,
        &xdg,
    );

    let proof_path = home_dir_path.join("test-lib/test-proj/proof.txt");
    assert!(proof_path.exists());

    let proof_contents = fs::read_to_string(&proof_path).unwrap();
    println!("{}", proof_contents);
    assert_eq!(
        proof_contents,
        format!(
            "{}, {}, {}, {}\n",
            "test-proj",
            home_dir_path.join("test-lib/test-proj").to_str().unwrap(),
            "test-type",
            "test-lib"
        )
    );

    let project_config = ProjectConfig::load(
        home_dir_path
            .join("test-lib/test-proj")
            .join(ProjectConfig::PROJECT_ROOT_REL_PATH)
            .to_str()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(project_config.project_type.as_deref(), Some("test-type"));
    assert_eq!(
        project_config.builder.as_deref(),
        home_dir_path.join("builder.lua").to_str()
    );
    assert_eq!(project_config.opener.as_deref(), None);
}
