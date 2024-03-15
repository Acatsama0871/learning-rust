use catr;
use std::process;

fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprint!("{}", e);
        process::exit(1);
    }
}
