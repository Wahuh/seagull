mod cli;
mod config;
mod init;
mod migrate;
mod poop;

use cli::Seagull;
use std::process;
use structopt::StructOpt;

fn main() {
    let args = Seagull::from_args();

    match cli::run(args) {
        Ok(()) => process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
