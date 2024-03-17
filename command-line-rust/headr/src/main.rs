use headr;
use std::process;

fn main() {
    if let Err(e) = headr::get_args().and_then(headr::run) {
        eprint!("{}", e);
        process::exit(1);
    }
}
