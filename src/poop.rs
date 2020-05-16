use anyhow::{Context, Result};
use colored::Colorize;
use std::{
    fs::{self, File},
    io,
    path::PathBuf,
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

fn get_next_version_number(dir_path: &PathBuf) -> Result<i32> {
    let mut files = fs::read_dir(dir_path)
        .with_context(|| {
            format!("Failed to read the files at {}. Either the path does not exist, the path is not a directory or you do not have permission to access the directory.", dir_path.display())
        })?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    files.sort();

    let next_version_number = match files.last() {
        Some(file) => {
            let current_version_number = file
                .file_name()
                .with_context(|| format!("Failed to get filename for {}", file.display()))?
                .to_str()
                .with_context(|| format!("Failed to convert filename to str {}", file.display()))?
                .chars()
                .nth(1)
                .with_context(|| {
                    format!(
                        "Filename for {} is invalid. It should look similar to `V1__inital.sql`",
                        file.display()
                    )
                })?
                .to_digit(10)
                .with_context(|| {
                    format!(
                        "Filename for {} is invalid. It should look similar to `V1__inital.sql`",
                        file.display()
                    )
                })?;

            current_version_number as i32 + 1
        }
        None => 1,
    };
    Ok(next_version_number)
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
