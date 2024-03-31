use uniqr;
use std::process;

fn main() {
    if let Err(e) = uniqr::get_args().and_then(uniqr::run) {
        eprint!("{}", e);
        process::exit(1);
    }
}
