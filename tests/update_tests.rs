use donna::{
    create_alias_group, delete_alias_group, untrack_alias_group, update_alias_group, Config, XDG,
};

mod utils;
use utils::{gen_test_home_path, setup_home};

#[test]
fn test_alias_group_update() {
    let unique_name = "test_alias_group_update";
    let unique_config_home_name = unique_name.to_string() + "_config";
    let unique_data_home_name = unique_name.to_string() + "_data";
    let xdg = XDG::new(
        Some(unique_name),
        Some(&unique_config_home_name),
        Some(&unique_data_home_name),
    );
    let _cleanup = setup_home(unique_name, &xdg);

    let home_dir_path = gen_test_home_path(unique_name);
    let alias_group_path1 = home_dir_path.join("group1");
    create_alias_group("group1", alias_group_path1.to_str().unwrap(), false, &xdg).unwrap();
    let alias_group_path2 = home_dir_path.join("group2");
    update_alias_group(
        "group1",
        None,
        Some(alias_group_path2.to_str().unwrap()),
        &xdg,
    )
    .unwrap();

    assert!(
        alias_group_path2.exists(),
        "The updated alias group path does not exist"
    );
    assert!(
        !alias_group_path1.exists(),
        "The old alias group path still exists"
    );
    let config = Config::load(None, &xdg).expect("Could not load config");
    let alias_group = config
        .get_alias_group("group1")
        .expect("Could not find alias group");
    assert_eq!(
        alias_group.path,
        alias_group_path2.to_str().unwrap(),
        "The alias group path was not updated correctly"
    );
    let alias_group = config.get_alias_group("group2");
    assert!(
        alias_group.is_none(),
        "The old alias group name still exists"
    );

    let alias_group_path3 = home_dir_path.join("group3");
    create_alias_group("group3", alias_group_path3.to_str().unwrap(), false, &xdg).unwrap();
    let alias_group_path4 = home_dir_path.join("group4");
    update_alias_group(
        "group3",
        Some("group4"),
        Some(alias_group_path4.to_str().unwrap()),
        &xdg,
    )
    .unwrap();
    assert!(
        alias_group_path4.exists(),
        "The updated alias group path does not exist"
    );
    assert!(
        !alias_group_path3.exists(),
        "The old alias group path still exists"
    );
    let config = Config::load(None, &xdg).expect("Could not load config");
    let alias_group = config
        .get_alias_group("group4")
        .expect("Could not find alias group");
    assert_eq!(
        alias_group.path,
        alias_group_path4.to_str().unwrap(),
        "The alias group path was not updated correctly"
    );
    let alias_group = config.get_alias_group("group3");
    assert!(
        alias_group.is_none(),
        "The old alias group name still exists"
    );
}

#[test]
fn test_untrack_alias_group() {
    let unique_name = "test_untrack_alias_group";
    let unique_config_home_name = unique_name.to_string() + "_config";
    let unique_data_home_name = unique_name.to_string() + "_data";
    let xdg = XDG::new(
        Some(unique_name),
        Some(&unique_config_home_name),
        Some(&unique_data_home_name),
    );
    let _cleanup = setup_home(unique_name, &xdg);

    let home_dir_path = gen_test_home_path(unique_name);
    let alias_group_path = home_dir_path.join("group1");
    create_alias_group("group1", alias_group_path.to_str().unwrap(), false, &xdg).unwrap();
    untrack_alias_group("group1", &xdg).unwrap();

    assert!(
        alias_group_path.exists(),
        "The alias group path doesn't exists"
    );
    let config = Config::load(None, &xdg).expect("Could not load config");
    let alias_group = config.get_alias_group("group1");
    assert!(
        alias_group.is_none(),
        "The alias group still exists in the config"
    );
}

#[test]
fn test_delete_alias_group() {
    let unique_name = "test_delete_alias_group";
    let unique_config_home_name = unique_name.to_string() + "_config";
    let unique_data_home_name = unique_name.to_string() + "_data";
    let xdg = XDG::new(
        Some(unique_name),
        Some(&unique_config_home_name),
        Some(&unique_data_home_name),
    );
    let _cleanup = setup_home(unique_name, &xdg);

    let home_dir_path = gen_test_home_path(unique_name);
    let alias_group_path = home_dir_path.join("group1");
    let alias_group_path2 = home_dir_path.join("group2");
    create_alias_group("group1", alias_group_path.to_str().unwrap(), false, &xdg).unwrap();
    create_alias_group("group2", alias_group_path2.to_str().unwrap(), false, &xdg).unwrap();
    delete_alias_group("group1", &xdg).unwrap();

    assert!(
        !alias_group_path.exists(),
        "The alias group path still exists"
    );
    let config = Config::load(None, &xdg).expect("Could not load config");
    let alias_group = config.get_alias_group("group1");
    assert!(
        alias_group.is_none(),
        "The alias group still exists in the config"
    );

    assert!(
        alias_group_path2.exists(),
        "The alias group path still exists"
    );
    let alias_group = config.get_alias_group("group2");
    assert!(
        alias_group.is_some(),
        "The alias group2 does not exist in the config"
    );
}
