mod utils;
use assert_cmd::Command;
use std::fs::{self, File};
use utils::setup_test_dir_with_config;

#[test]
fn poop_creates_sql_file_at_default_dir() {
    let config = "";
    let test_dir = setup_test_dir_with_config(config);

    let mut cmd = Command::cargo_bin("seagull").unwrap();
    cmd.arg("poop")
        .arg("initial")
        .current_dir(test_dir.path())
        .assert()
        .success();

    let is_sql_file_created = test_dir
        .path()
        .join("migrations")
        .join("V1__initial.sql")
        .exists();
    assert!(is_sql_file_created);
}

#[test]
fn poop_creates_sql_file_at_dir_from_config() {
    let config = "
        [migrations] \n
        dir_path = \"src/migrations\" \n
    ";
    let test_dir = setup_test_dir_with_config(config);

    let mut cmd = Command::cargo_bin("seagull").unwrap();
    cmd.arg("poop")
        .arg("initial")
        .current_dir(test_dir.path())
        .assert()
        .success();

    let is_sql_file_created = test_dir
        .path()
        .join("src")
        .join("migrations")
        .join("V1__initial.sql")
        .exists();
    assert!(is_sql_file_created);
}

#[test]
fn poop_creates_sql_file_with_incremented_version_number_when_other_migrations_exist() {
    let config = "";
    let test_dir = setup_test_dir_with_config(config);
    let migrations_dir_path = test_dir.path().join("migrations");
    fs::create_dir_all(&migrations_dir_path).unwrap();
    File::create(migrations_dir_path.join("V1__initial.sql")).unwrap();
    File::create(migrations_dir_path.join("V2__create_cars_table.sql")).unwrap();

    let mut cmd = Command::cargo_bin("seagull").unwrap();
    cmd.arg("poop")
        .arg("create_users_table")
        .current_dir(test_dir.path())
        .assert()
        .success();

    let is_sql_file_created = test_dir
        .path()
        .join("migrations")
        .join("V3__create_users_table.sql")
        .exists();
    assert!(is_sql_file_created);
}
