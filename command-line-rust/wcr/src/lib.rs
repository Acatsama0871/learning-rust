use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

pub fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn run(config: Config) -> Result<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(reader) => {
                if let Ok(result) = count(reader) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(result.num_lines, config.lines),
                        format_field(result.num_words, config.words),
                        format_field(result.num_bytes, config.bytes),
                        format_field(result.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );

                    total_lines += result.num_lines;
                    total_words += result.num_words;
                    total_chars += result.num_chars;
                    total_bytes += result.num_bytes;
                }
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars),
        )
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> Result<Config> {
    let matches = Command::new("wcr")
        .version("0.0.0")
        .author("Haohang Li <hli113@stevens.edu>")
        .about("A rust naive version of 'wc'")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input files")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .value_name("LINES")
                .help("num of lines to output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .value_name("WORDS")
                .help("num of words to output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .help("num of bytes to output")
                .conflicts_with("chars")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .value_name("CHARS")
                .help("num of characters to output")
                .conflicts_with("bytes")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // parse arguments
    let files_arguments = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.clone())
        .collect::<Vec<_>>();
    let mut line_indicator = matches.get_flag("lines");
    let mut word_indicator = matches.get_flag("words");
    let mut byte_indicator = matches.get_flag("bytes");
    let mut char_indicator = matches.get_flag("chars");

    // map to true if all
    if [
        line_indicator,
        word_indicator,
        byte_indicator,
        char_indicator,
    ]
    .iter()
    .all(|v| v == &false)
    {
        line_indicator = true;
        word_indicator = true;
        byte_indicator = true;
        char_indicator = false;
    }

    Ok(Config {
        files: files_arguments,
        lines: line_indicator,
        words: word_indicator,
        bytes: byte_indicator,
        chars: char_indicator,
    })
}
