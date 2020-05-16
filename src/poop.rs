use anyhow::{Context, Result};
use colored::Colorize;
use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

pub fn handle_poop(description: String, dir_path: String) -> Result<()> {
    let dir_path = PathBuf::from(&dir_path);
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path)?;
        let filename = generate_filename(&description, 1);
        create_sql_file(filename, dir_path)?;
    } else {
        let next_version_number = get_next_version_number(&dir_path)?;
        let filename = generate_filename(&description, next_version_number);
        create_sql_file(filename, dir_path)?;
    }
    Ok(())
}

fn generate_filename(description: &str, version_number: i32) -> String {
    let version = format!("V{}", version_number);
    let separator = "__";
    let extension = ".sql";

    let filename = format!("{}{}{}{}", version, separator, description, extension);
    filename
}

fn get_next_version_number<P: AsRef<Path>>(dir_path: P) -> Result<i32> {
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

fn create_sql_file(file_name: String, dir_path: PathBuf) -> Result<()> {
    let file_path = dir_path.join(&file_name);
    println!("{}", file_path.display());
    File::create(&file_path)
        .with_context(|| format!("Failed to create {}.", file_path.display()))?;
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
    println!(
        "{}{}",
        "Created: ".green().bold(),
        file_path.to_str().unwrap_or(&file_name).green()
    );
    Ok(())
}
