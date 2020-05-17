use postgres::{Client, NoTls};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use tempfile::TempDir;

pub struct TestDatabase {
    pub connection_string: String,
    client: Client,
}

impl TestDatabase {
    pub fn connect() -> TestDatabase {
        let connection_string =
            String::from("postgres://postgres:postgres@localhost:5432/postgres");
        TestDatabase {
            client: Client::connect(&connection_string, NoTls).unwrap(),
            connection_string,
        }
    }

    pub fn clean(&mut self) {
        let sql = "
            DROP SCHEMA public CASCADE;
            CREATE SCHEMA public;
            GRANT ALL ON SCHEMA public TO postgres;
            GRANT ALL ON SCHEMA public TO public;
            COMMENT ON SCHEMA public IS 'standard public schema';
        ";
        &self.client.batch_execute(sql);
    }

    pub fn assert_tables_exist(&mut self, table_names: Vec<&str>) {
        for table_name in table_names {
            let sql = "
                SELECT EXISTS (
                    SELECT FROM information_schema.tables 
                    WHERE table_name = ($1)
                );
            ";
            let row = self.client.query_one(sql, &[&table_name]).unwrap();
            let exists: bool = row.get(0);
            assert!(exists);
        }
    }

    pub fn assert_migration_history_exists(&mut self, version_numbers: Vec<i32>) {
        let sql = "
            SELECT version_number
            FROM __migration_history
        ";
        let rows = self.client.query(sql, &[]).unwrap();
        for (i, row) in rows.iter().enumerate() {
            let version_number: i32 = row.get(0);
            assert_eq!(version_number, version_numbers[i]);
        }
    }
}

pub struct TestDir {
    tempdir: TempDir,
    migrations_dir_path: PathBuf,
}

impl TestDir {
    pub fn new() -> TestDir {
        TestDir {
            tempdir: TempDir::new().unwrap(),
            migrations_dir_path: PathBuf::from("migrations"),
        }
    }

    pub fn assert_exists(&self, filename: &str) {
        let exists = self
            .tempdir
            .path()
            .join(&self.migrations_dir_path)
            .join(filename)
            .exists();
        assert!(exists);
    }

    pub fn path(&self) -> &Path {
        self.tempdir.path()
    }

    pub fn with_toml_file(mut self, migrations_dir_path: &str) -> Self {
        let config = format!(
            "
            [migrations] \n
            dir_path = \"{}\" \n
        ",
            migrations_dir_path
        );
        let config_path = self.tempdir.path().join("seagull.toml");
        let mut config_file = File::create(config_path).unwrap();
        config_file.write_all(config.as_bytes()).unwrap();
        self.migrations_dir_path = PathBuf::from(migrations_dir_path);
        self
    }

    pub fn with_migrations(self, fixtures_dir: &str) -> Self {
        let migrations_path = self.tempdir.path().join("migrations");
        fs::create_dir_all(&migrations_path).unwrap();

        let dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(fixtures_dir);
        let entries = fs::read_dir(dir_path).unwrap();

        for entry in entries {
            let fixture_path = entry.unwrap().path();
            let test_filepath = migrations_path.join(fixture_path.file_name().unwrap());
            File::create(&test_filepath).unwrap();
            fs::copy(fixture_path, &test_filepath).unwrap();
        }
        self
    }
}
