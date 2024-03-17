use clap::{Arg, ArgAction, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config {
        files: vec![String::from("value")],
        lines: 8,
        bytes: Some(13),
    })
}
