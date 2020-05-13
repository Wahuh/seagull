use anyhow::{Context, Result};
use colored::Colorize;
use std::{
    fs::{self, File},
    io,
    path::Path,
};

pub fn get_next_version_number<P: AsRef<Path>>(dir_path: P) -> Result<i32> {
    let mut files = fs::read_dir(dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    files.sort();

    match files.last() {
        Some(path) => {
            let n = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .chars()
                .nth(1)
                .unwrap()
                .to_digit(10)
                .unwrap();
            return Ok(n as i32 + 1);
        }
        None => return Ok(0),
    };
}

pub fn create_sql_file(description: &str, version_number: i32) -> Result<()> {
    let version = format!("V{}", version_number);
    let separator = "__";
    let extension = ".sql";

    let file_name = format!(
        "migrations/{}{}{}{}",
        version, separator, description, extension
    );

    File::create(&file_name).with_context(|| format!("Failed to create {}.", file_name))?;
    println!(
        "
    ░░░░░▄▀▀▀▄░░░░░░░░░░░░░░░░░
    ▄███▀░◐░░░▌░░░░░░░░░░░░░░░░
    ░░░░▌░░░░░▐░░░░░░░░░░░░░░░░
    ░░░░▐░░░░░▐░░░░░░░░░░░░░░░░
    ░░░░▌░░░░░▐▄▄░░░░░░░░░░░░░░
    ░░░░▌░░░░▄▀▒▒▀▀▀▀▄░░░░░░░░░
    ░░░▐░░░░▐▒▒▒▒▒▒▒▒▀▀▄░░░░░░░
    ░░░▐░░░░▐▄▒▒▒▒▒▒▒▒▒▒▀▄░░░░░
    ░░░░▀▄░░░░▀▄▒▒▒▒▒▒▒▒▒▒▀▄░░░
    ░░░░░░▀▄▄▄▄▄█▄▄▄▄▄▄▄▄▄▄▄▀▄░
    ░░░░░░░░░░░▌▌░▌▌░░░░░░░░░░░
    ░░░░░░░░░░░▌▌░▌▌░░░░░░░░░░░
    ░░░░░░░░░▄▄▌▌▄▌▌░░░░░░░░░░░
    "
    );
    println!("{}{}", "Created: ".green().bold(), file_name.green());
    Ok(())
}
