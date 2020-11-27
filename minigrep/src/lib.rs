use std::env;
use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line);
    //     }
    // }
    //
    // result
    contents.lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase()))
        .collect()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    //
    // result
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}