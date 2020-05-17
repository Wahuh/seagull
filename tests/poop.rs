mod utils;
use assert_cmd::Command;
use utils::TestDir;

#[test]
fn poop_creates_sql_file_at_default_dir() {
    let test_dir = TestDir::new().with_toml_file("migrations");

    let mut cmd = Command::cargo_bin("seagull").unwrap();
    cmd.arg("poop")
        .arg("initial")
        .current_dir(test_dir.path())
        .assert()
        .success();
    test_dir.assert_exists("V1__initial.sql");
}

#[test]
fn poop_creates_sql_file_at_dir_from_config() {
    let test_dir = TestDir::new().with_toml_file("src/migrations");

    let mut cmd = Command::cargo_bin("seagull").unwrap();
    cmd.arg("poop")
        .arg("initial")
        .current_dir(test_dir.path())
        .assert()
        .success();
    test_dir.assert_exists("V1__initial.sql");
}

#[test]
fn poop_creates_sql_file_with_incremented_version_number_when_other_migrations_exist() {
    let test_dir = TestDir::new()
        .with_toml_file("migrations")
        .with_migrations("migrations_all_unapplied");

    let mut cmd = Command::cargo_bin("seagull").unwrap();
    cmd.arg("poop")
        .arg("create_users_table")
        .current_dir(test_dir.path())
        .assert()
        .success();

    test_dir.assert_exists("V3__create_users_table.sql");
}
