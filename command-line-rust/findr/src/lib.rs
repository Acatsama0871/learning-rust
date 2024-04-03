use crate::EntryType::*;
use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use regex::Regex;
use walkdir::WalkDir;
// use std::fs::File;
// use std::io::{self, BufRead, BufReader};

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
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
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .short('n')
                .long("name")
                .help("name")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .short('t')
                .long("type")
                .value_parser(["f", "d", "l"])
                .help("Entry type")
                .num_args(1..)
                .required(true),
        )
        .get_matches();

    // parsing the parameters
    let paths = matches
        .get_many::<String>("paths")
        .unwrap_or_default()
        .map(|s| s.clone())
        .collect::<Vec<_>>();
    let names = matches
        .get_many::<String>("names")
        .unwrap_or_default()
        .map(|s| Regex::new(s).map_err(|_| anyhow!(format!("Invalid --name \"{}\"", s))))
        .collect::<Result<Vec<_>, _>>()?;
    let types = matches
        .get_many::<String>("types")
        .unwrap_or_default()
        .map(|s| type_value_parser(s))
        .collect::<Vec<_>>();

    Ok(Config {
        paths: paths,
        names: names,
        entry_types: types,
    })
}

pub fn run(config: Config) -> Result<()> {
    for cur_path in config.paths {
        for entry in WalkDir::new(cur_path) {
            match entry {
                Err(e) => Err(anyhow!(e))?,
                Ok(entry) => {
                    for cur_reg in &config.names {
                        if cur_reg.is_match(entry.path().to_str().unwrap()) {
                            println!("{}", entry.path().display());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn type_value_parser(s: &str) -> EntryType {
    match s {
        "d" => Dir,
        "f" => File,
        "l" => Link,
        _ => unreachable!("You should never be here"),
    }
}
