use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for file_name in config.files {
        match open(&file_name) {
            Err(err) => eprint!("Failed to open {}: {}", file_name, err),
            Ok(reader) => {
                let mut counter = 0;
                for line in reader.lines() {
                    if config.number_lines {
                        counter += 1;
                        println!("{:>6}\t{}", counter, line?);
                    } else if config.number_nonblank_lines {
                        let line = line?;
                        if !line.is_empty() {
                            counter += 1;
                            println!("{:>6}\t{}", counter, line);
                        } else {
                            println!();
                        }
                    } else {
                        let line = line?;
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.0.0")
        .author("Haohang Li <hli113@stevens.edu>")
        .about("Rust Cat")
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("number lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("number non-blank lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                .default_value("-")
                .num_args(1..),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .map(|s| s.clone())
            .collect::<Vec<_>>(),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    })
}
