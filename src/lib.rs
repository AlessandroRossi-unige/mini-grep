use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };
    println!("Found {} results:", results.len());

    for result in results {
        println!("{}", result)
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_name = match args.next() {
            Some(f) => f,
            None => return Err("Didn't get a file name"),
        };
        match args.next() {
            Some(_t) => return Err("Unexpected argument"),
            None => {
                Ok(
                    Config{
                        query,
                        file_name,
                        case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
                    }
                )
            },
        }
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line|line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line|line.to_lowercase().contains(query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_result() {
        let query = "apple";
        let contents = "Rust:\nsafe, fast, productive\nPick three.";
        let empty: Vec<&str> = vec![];
        assert_eq!(empty, search(query, contents));
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive\nPick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }

    #[test]
    fn two_results() {
        let query = "st";
        let contents = "Rust:\nsafe, fast, productive\nPick three.";
        assert_eq!(vec!["Rust:", "safe, fast, productive"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive\nPick three.\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_insensitive(query, contents));
    }
}
