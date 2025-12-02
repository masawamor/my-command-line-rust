use std::process;

fn main() {
    // println!("### Hello, world!");
    
    if let Err(err) = commr::get_args().and_then(commr::run) {
        eprintln!("{}", err);
        process::exit(1);
    }
    
    // println!("### main suceceed");
}
