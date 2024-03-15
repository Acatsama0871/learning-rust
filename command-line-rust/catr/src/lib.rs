use std::error::Error;
use clap::{Command, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

#[derive(Debug)]
pub struct  Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
    .version("0.0.0")
    .author("Haohang Li <hli113@stevens.edu>")
    .about("Rust Cat")
    .get_matches();

    Ok(Config {
        files: vec![String::from("new")],
        number_lines: false,
        number_nonblank_lines: false
    })
}
