use crate::EntryType::*;
use anyhow::{anyhow, Result};
use clap::{Arg, ArgAction, Command};
use regex::Regex;
// use std::fs::File;
// use std::io::{self, BufRead, BufReader};


#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>
}

pub fn get_args() -> Result<Config> {
    let matches = Command::new("findr")
    .version("0.0.0")
    .author("Haohang Li <haohang.li@acatsama.io>")
    .about("A rust naive implementation of BSD 'find'.")
    .arg(
        Arg::new("paths")
        .value_name("PATH")
        .help("Search paths")
        .default_value(".")
        .num_args(1..)
        .required(true)
    )
    .arg(
        Arg::new("names")
        .value_name("NAME")
        .short('n')
        .long("name")
        .help("name")
        .num_args(1..)
        .required(true)
    )
    .arg(
        Arg::new("types")
        .value_name("TYPE")
        .short('t')
        .long("type")
        .value_parser(["f", "d", "l"])
        .help("Entry type")
        .num_args(1..)
        .required(true)
    )
    .get_matches();

    Ok(
        Config {
            paths: vec![String::from("test")],
            names: vec![],
            entry_types: vec![Dir]
            
        }
    )
}

pub fn run(config: Config) -> Result<()> {
    dbg!(&config);
    Ok(())
}
