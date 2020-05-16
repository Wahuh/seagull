use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

#[derive(Deserialize, Serialize)]
pub struct Postgres {
    #[serde(default)]
    pub host: String,

    #[serde(default)]
    pub password: String,

    #[serde(default)]
    pub port: i32,

    #[serde(default)]
    pub username: String,

    #[serde(default)]
    pub database: String,
}

impl Postgres {
    pub fn connection_string(&self) -> String {
        String::from(format!(
            "host={} user={} password={} port={} dbname={}",
            self.host, self.username, self.password, self.port, self.database
        ))
    }
}

#[derive(Deserialize, Serialize)]
pub struct Migrations {
    #[serde(default)]
    pub dir_path: String,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub migrations: Migrations,

    #[serde(default)]
    pub postgres: Postgres,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let contents = fs::read_to_string(path).with_context(|| {
            format!("Failed to find seagull.toml config file. Run `seagull init` to create it.")
        })?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn from_input() -> Result<Config> {
        let mut config = Config::default();

        print!("What's your database host? (localhost) ");
        io::stdout().flush()?;
        let mut host = String::new();
        io::stdin().read_line(&mut host)?;

        print!("What's your database username? (postgres) ");
        io::stdout().flush()?;
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;

        print!("What's your database password? ");
        io::stdout().flush()?;
        let mut password = String::new();
        io::stdin().read_line(&mut password)?;

        print!("What's your database port? (5432) ");
        io::stdout().flush()?;
        let mut port_string = String::new();
        io::stdin().read_line(&mut port_string)?;

        if !port_string.trim().is_empty() {
            let port: i32 = port_string
                .trim()
                .parse()
                .expect("Please enter a positive port number");
            config.postgres.port = port;
        }

        print!("What's your database name? (postgres) ");
        io::stdout().flush()?;
        let mut database = String::new();
        io::stdin().read_line(&mut database)?;

        if !host.trim().is_empty() {
            config.postgres.host = host.trim().to_string();
        };

        if !username.trim().is_empty() {
            config.postgres.username = username.trim().to_string();
        }
        if !password.trim().is_empty() {
            config.postgres.password = password.trim().to_string();
        }

        if !database.trim().is_empty() {
            config.postgres.database = database.trim().to_string()
        }

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            migrations: Migrations::default(),
            postgres: Postgres::default(),
        }
    }
}

impl Default for Postgres {
    fn default() -> Self {
        Postgres {
            host: String::from("localhost"),
            username: String::from("postgres"),
            password: String::from("postgres"),
            port: 5432,
            database: String::from("postgres"),
        }
    }
}

impl Default for Migrations {
    fn default() -> Self {
        Migrations {
            dir_path: String::from("migrations"),
        }
    }
}
