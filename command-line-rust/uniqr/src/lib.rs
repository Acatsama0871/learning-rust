use anyhow::Result;
use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> Result<Config> {
    let matches = Command::new("uniqr")
        .version("0.0.0")
        .author("Haohang Li <hli113@stevens.edu>")
        .author("A Rust Naive Implementation of BSD 'uniq'")
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input files")
                .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .help("Output files"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("whether to show the count")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // get match results
    let in_file = matches.get_one::<String>("in_file").unwrap().clone();
    let out_file = matches.get_one::<String>("out_file").map(String::from);
    let show_count = matches.get_flag("count");

    Ok(Config {
        in_file: in_file,
        out_file: out_file,
        count: show_count,
    })
}

pub fn run(config: Config) -> Result<()> {
    dbg!(&config);

    Ok(())
}
