mod utils;

use assert_cmd::Command;
use utils::{TestDatabase, TestDir};

#[test]
fn migrate_applies_fresh_migrations_using_args() {
    let test_dir = TestDir::new()
        .with_toml_file("migrations")
        .with_migrations("migrations_all_unapplied");

    let mut test_database = TestDatabase::connect();
    test_database.clean();

    let mut cmd = Command::cargo_bin("seagull").unwrap();
    cmd.arg("migrate")
        .arg("--url")
        .arg(&test_database.connection_string)
        .current_dir(test_dir.path())
        .assert()
        .success();

    test_database.assert_migration_history_exists(vec![1, 2]);
    test_database.assert_tables_exist(vec!["users", "comments", "issues", "projects"]);
}
