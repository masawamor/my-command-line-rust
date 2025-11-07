use std::{error::Error, io::{self, BufRead, BufReader, Write}};

use clap::Arg;

pub mod Const {
    pub const PRG: &str = "uniqr";
    pub const ARG_IN_FILE: &str = "in_file";
    pub const ARG_OUT_FILE: &str = "out_file";
    pub const ARG_COUNT: &str = "count";
}

use Const::*;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

struct PrevItem {
    line: String,
    count: u32,
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
    
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e) )?;
    
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(filename) => Box::new(std::fs::File::create(filename)?),
        None => Box::new(io::stdout()),
    };
    
    let mut prev_item = PrevItem { line: "".to_string(), count: 0 };
    
    let mut line = String::new();
    
    let mut print = |prev_item: &PrevItem| -> MyResult<()> {
        if prev_item.count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", prev_item.count, prev_item.line)?;
            } else {
                write!(out_file, "{}", prev_item.line)?;
            }
        }
        Ok(())
    };
    
    loop {
        
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        
        if prev_item.line.trim_end() != line.trim_end() {
            print(&prev_item);
            
            prev_item.line = line.clone();
            prev_item.count = 1;
        } else {
            prev_item.count += 1;
        }
        line.clear();
    }
    
    print(&prev_item);
    
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(filename)?)))
    }
}

// ############################################################################



