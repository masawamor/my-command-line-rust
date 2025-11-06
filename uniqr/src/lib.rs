use std::error::Error;

use clap::Arg;

const PRG: &str = "uniqr";
const ARG_IN_FILE: &str = "in_file";
const ARG_OUT_FILE: &str = "out_file";
const ARG_COUNT: &str = "count";

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

// ############################################################################

pub fn get_args() -> MyResult<Config> {
    
    let matches = clap::App::new(PRG)
        .version("0.1.0")
        .author("masawamor")
        .about("Rust uniqr")
        .arg(
            Arg::with_name(ARG_IN_FILE)
                .value_name("IN_FILE")
                .help("Input file")
                .multiple(false)
                .default_value("-")
        )
        .arg(
            Arg::with_name(ARG_OUT_FILE)
                .value_name("OUT_FILE")
                .help("Output file")
                .multiple(false)
        )
        .arg(
            Arg::with_name(ARG_COUNT)
                .short("c")
                .long("count")
                .value_name("OUT_FILE")
                .help("Show counts")
                .multiple(false)
                .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        in_file: matches.value_of_lossy(ARG_IN_FILE).unwrap().to_string(),
        out_file: matches.value_of(ARG_OUT_FILE).map(String::from),
        count: matches.is_present(ARG_COUNT),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

// ############################################################################



