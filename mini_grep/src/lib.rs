use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, Box<dyn Error>> {

        if args.len() < 3 {
            return Err("Not enough arguments".into());
        }

        let conf: Config = Config {
            query: args[1].clone(),
            file_name: args[2].clone(),
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
        println!("hey");
        case_insensitive_search(&contents, &conf.query);
    }else {
        println!("ho");
        search(&contents, &conf.query);
    }

    Ok(())
}

pub fn search(content: &str, query: &str) {
    for line in content.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
}

pub fn case_insensitive_search(content: &str, query: &str) {
    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            println!("{}", line);
        }
    }
}