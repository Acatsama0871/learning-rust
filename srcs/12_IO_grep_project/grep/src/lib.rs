use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();
        
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get file path")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config {query, file_path, ignore_case})
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    // get the file
    let content = fs::read_to_string(&config.file_path)?;
    // search
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    
    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let contents = "\
            Rust:
safe, fast, productive.
Pick three.
Duck Tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn test_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
