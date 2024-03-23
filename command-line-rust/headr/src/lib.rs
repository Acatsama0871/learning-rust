use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::{error::Error, usize};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut reader) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                match config.bytes {
                    Some(byte_limit) => {
                        eprint!("Not implemented!")
                    }
                    None => {
                        let mut line = String::new();
                        for _ in 0..config.lines {
                            let bytes = reader.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            print!("{}", line.trim_end_matches('\r'));
                            line.clear();
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.0.0")
        .author("Haohang Li <hli113@stevens.edu>")
        .about("rust head")
        .arg(
            Arg::new("LINES")
                .short('n')
                .long("lines")
                .help("num of lines")
                .conflicts_with("BYTES")
                .default_value("10")
                .num_args(1),
        )
        .arg(
            Arg::new("BYTES")
                .short('c')
                .long("bytes")
                .help("num-bytes")
                .num_args(1),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("input files")
                .default_value("-")
                .num_args(1..),
        )
        .get_matches();

    // match arguments
    let file_arguments = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.clone())
        .collect::<Vec<_>>();
    let lines = parse_positive_args(matches.get_one::<String>("LINES").unwrap().as_str())
        .map_err(|e| format!("illegal line count -- {}", e))?;
    let bytes = match matches.get_one::<String>("BYTES") {
        Some(a) => {
            Some(parse_positive_args(a).map_err(|e| format!("illegal byte count -- {}", e))?)
        }
        None => None,
    };

    Ok(Config {
        files: file_arguments,
        lines: lines,
        bytes: bytes,
    })
}

fn parse_positive_args(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_args("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_args("foo");
    assert!(res.is_err());

    let res = parse_positive_args("0");
    assert!(res.is_err());
}
