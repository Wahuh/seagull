use assert_cmd::Command;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

#[test]
fn it_creates_default_migrations_dir() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("seagull.toml");
    let mut temp_file = File::create(file_path).unwrap();
    let config = b"[database] \n
    connection_string = \"Sqlite\" \n";
    temp_file.write_all(config).unwrap();

    let mut cmd = Command::cargo_bin("seagull").unwrap();
    let assert = cmd
        .arg("poop")
        .arg("miggy")
        .current_dir(temp_dir.path())
        .assert();

    let is_migrations_dir_created = temp_dir.path().join("migrations").exists();
    assert!(is_migrations_dir_created);
    assert.success();
}

#[test]
fn it_creates_sql_migration_files() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("seagull.toml");
    let mut temp_file = File::create(file_path).unwrap();
    let config = b"[database] \n
    connection_string = \"Sqlite\" \n";
    temp_file.write_all(config).unwrap();

    let mut cmd = Command::cargo_bin("seagull").unwrap();
    let assert = cmd
        .arg("poop")
        .arg("miggy")
        .current_dir(temp_dir.path())
        .assert();

    let migration_file_path = temp_dir.path().join("migrations/V1__miggy.sql");
    assert!(migration_file_path.exists());
    assert.success();
}
