use std::{env, process};
use mini_grep::Config;

fn main() {
    eprintln!("Welcome to mini grep!");

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });
    eprintln!("String to grep: {}\nFile path: {}", config.query, config.file_name);

    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


