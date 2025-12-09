use std::process;

fn main() {
    if let Err(err) = tailr::get_args().and_then(tailr::run) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
