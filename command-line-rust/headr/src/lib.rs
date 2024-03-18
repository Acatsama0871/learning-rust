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
    let matches = Command::new("headr")
    .version("0.0.0")
    .author("Haohang Li <hli113@stevens.edu>")
    .about("rust head")
    .arg(
        Arg::new("num_lines")
        .short('n')
        .long("num-lines")
        .help("num of lines")
        .conflicts_with("num-bytes")
        .default_value("10")
        .num_args(1)
    )
    .arg (
        Arg::new("num-bytes")
        .short('b')
        .long("num-bytes")
        .help("num-bytes")
        .num_args(1)
    )
    .arg(
        Arg::new("files")
        .value_name("FILE")
        .help("input files")
        .default_value("-")
        .num_args(2..)
    ).get_matches();
    
    // match arguments
    let file_arguments = matches.get_many::<String>("files").unwrap().map(|s| s.clone()).collect::<Vec<_>>();
    let lines = parse_positive_args(matches.get_one::<String>("num_lines"))?;
    
    println!("{}", lines);
    
    Ok(Config {
        files: file_arguments,
        lines: lines,
        bytes: Some(13),
    })
}


fn parse_positive_args(val: Option<&String>) -> MyResult<usize> {
    match val {
        Some(a) => match a.parse() {
            Ok(n) if n > 0 => Ok(n),
            _ => Err(format!("Invalid value '{}' for 'num_lines'", a).into()),
        }
        None => Ok(10)
    }
}

