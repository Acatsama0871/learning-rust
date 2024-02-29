use std::fs;
use std::error::Error;

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("The arguments' length should be 3.");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config {query, file_path})
    }
}

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>>{
    // load config
    let config = Config::build(&args)?;
    // get the file
    let content = fs::read_to_string(&config.file_path)?;
    println!("With text:\n{}", content);
    
    Ok(())
}
