use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use std::error::Error;
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

pub fn run(config: Config) -> Result<()> {
    println!("{:#?}", config);
    Ok(())
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
                .value_name("LINES")
                .help("num of lines to output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .value_name("WORDS")
                .help("num of words to output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .value_name("BYTES")
                .help("num of bytes to output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .value_name("CHARS")
                .help("num of characters to output")
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
        char_indicator = true;
    }

    Ok(Config {
        files: files_arguments,
        lines: line_indicator,
        words: word_indicator,
        bytes: byte_indicator,
        chars: char_indicator,
    })
}
