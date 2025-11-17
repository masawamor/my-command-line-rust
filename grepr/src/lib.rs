use std::error::Error;

use clap::Arg;
use regex::{Regex, RegexBuilder};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}

// ############################################################################

pub fn get_args() -> MyResult<Config> {
    let matches = clap::App::new("grepr")
        .version("0.1.0")
        .author("author aaa")
        .about("Rust grep")
        .arg(
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .help("Search pattern")
                .multiple(false)
                .required(true)
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("recursive")
                .help("Recursive search")
                .short("r")
                .long("recursive")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("count")
                .value_name("COUNT")
                .help("Count occurrences")
                .short("c")
                .long("count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("invert")
                .help("Invert match")
                .short("v")
                .long("invert-match")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("insensitive")
                .help("Case-insensitive")
                .short("i")
                .long("insensitive")
                .takes_value(false)
        )
        .get_matches();
    
    let pattern_str = matches.value_of("pattern").unwrap();
    let pattern = RegexBuilder::new(pattern_str)
        .case_insensitive(matches.is_present("insensitive"))
        .build()
        .map_err(|_| format!("Invalid pattern  \"{}\"", pattern_str))?;
    
    let files = matches.values_of_lossy("files").unwrap();
    
    Ok(
        Config {
            pattern,
            files,
            recursive: matches.is_present("recursive"),
            count: matches.is_present("count"),
            invert_match: matches.is_present("invert"),
        }
    )
}

pub fn run(config: Config) -> MyResult<()> {
    println!("config: {:?}", config);
    
    Ok(())
}
