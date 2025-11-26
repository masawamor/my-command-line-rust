
use std::error::Error;

use clap::Arg;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}

// ############################################################################

pub fn get_args() -> MyResult<Config> {
    let matches = clap::App::new("commr")
        .version("0.1.0")
        .author("masawamor")
        .about("Rust comm")
        .arg(
            Arg::with_name("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .required(true)
                .takes_value(true)
                .multiple(false)
        )
        .arg(
            Arg::with_name("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .required(true)
                .takes_value(true)
                .multiple(false)
        )
        .arg(
            Arg::with_name("suppress_col1")
                .short("1")
                .help("suppress column 1 (lines unique to FILE1)")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("suppress_col2")
                .short("2")
                .help("suppress column 2 (lines unique to FILE2)")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("suppress_col3")
                .short("3")
                .help("suppress column 3 (lines that appears in both files)")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("insensitive")
                .short("i")
                .help("Case-insensitive comparison of lines")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("output-delimiter")
                .value_name("DELIM")
                .help("Output delimiter")
                .default_value("\t")
                .takes_value(true)
                .multiple(false)
        )
        .get_matches();
    
    let file1 = matches.value_of("file1").unwrap().to_string();
    let file2 = matches.value_of("file2").unwrap().to_string();
    let show_col1 = !matches.is_present("suppress_col1");
    let show_col2 = !matches.is_present("suppress_col2");
    let show_col3 = !matches.is_present("suppress_col3");
    let insensitive = matches.is_present("insensitive");
    let delimiter = matches.value_of("delimiter").unwrap().to_string();
    
    Ok(Config {
        file1,
        file2,
        show_col1,
        show_col2,
        show_col3,
        insensitive,
        delimiter,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    
    Ok(())
}