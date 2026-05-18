use std::error::Error;
use std::fs;
use std::env;
use std::env::Args;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new (mut args: env::Args) -> Result<Config, Box<dyn Error>> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string".into()),
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name".into()),
        };

        let conf: Config = Config {
            query,
            file_name,
            ignore_case : match env::var("IGNORE_CASE") {
            Ok(val) => val.to_lowercase() != "false", // If it's "false", this becomes false
            Err(_) => false, // If the variable isn't set at all, default to false
            }
        };

        Ok(conf)
    }
}

pub fn run (conf: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(conf.file_name)?;

    if conf.ignore_case {
        case_insensitive_search(&contents, &conf.query);
    }else {
        search(&contents, &conf.query);
    }

    Ok(())
}

pub fn search(content: &str, query: &str) {
    content.lines().filter(|line| line.contains(query)).for_each(|line| println!("{}", line))
}

pub fn case_insensitive_search(content: &str, query: &str) {
    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            println!("{}", line);
        }
    }
    content.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).for_each(|line| println!("{}", line))
}