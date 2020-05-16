use std::{fs::File, io::Write};
use tempfile::TempDir;

pub fn setup_test_dir_with_config(config: &str) -> TempDir {
    let test_dir = TempDir::new().unwrap();
    let config_path = test_dir.path().join("seagull.toml");
    let mut config_file = File::create(config_path).unwrap();
    config_file.write_all(config.as_bytes()).unwrap();
    test_dir
}
