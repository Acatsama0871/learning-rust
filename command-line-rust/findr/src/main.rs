use findr;
use std::process;

fn main() {
    if let Err(e) = findr::get_args().and_then(findr::run) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
