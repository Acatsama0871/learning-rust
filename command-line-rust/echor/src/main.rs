use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
    .version("0.0.0")
    .author("Haohang Li <hli113.stevens.edu>")
    .about("rust echo")
    .arg(
        Arg::new("omit_newline")
        .short('n')
        .help("Do not print newline")
        .action(ArgAction::SetTrue)
    )
    .arg(
        Arg::new("text")
        .value_name("TEXT")
        .help("Text input")
        .required(true)
        .num_args(1..)
    ).get_matches();

    
    // get matched values
    let text_values = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|s| s.as_str())
        .collect::<Vec<_>>();
    
    let omit_newline = matches.get_flag("omit_newline");
    
    println!("{:?}", text_values);
    println!("{:?}", omit_newline);
}
