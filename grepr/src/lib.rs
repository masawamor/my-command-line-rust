use std::{error::Error, fmt::format, fs, io::{self, BufRead, BufReader}, mem};

use clap::Arg;
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

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
    
    let entries = find_files(&config.files, config.recursive);
    let num_files = entries.len();
    
    let print_match = |filename: &str, val: &str| {
        if num_files > 1 {
            print!("{}:{}", filename, val);
        } else {
            print!("{}", val);
        }
    };
    
    for entry in entries {
        match entry {
            Err(find_files_err) => eprintln!("{}", find_files_err),
            Ok(filename) => {
                match open(&filename) {
                    Err(open_err) => eprintln!("{}", open_err),
                    Ok(file) => {
                        let matches = find_lines(file, &config.pattern, config.invert_match);
                        match matches {
                            Err(find_lines_err) => eprintln!("{}", find_lines_err),
                            Ok(matches) => {
                                if config.count {
                                    print_match(&filename, &format!("{}\n", matches.len()));
                                } else {
                                    for line in matches {
                                        print_match(&filename, &line);
                                    }
                                }
                            },
                        }
                    },
                }
            },
        }
    }
    
    Ok(())
}

// ############################################################################

fn find_lines<T: BufRead>(mut file: T, pattern: &Regex, invert_match: bool) -> MyResult<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();
    
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        
        if pattern.is_match(&line) ^ invert_match {
            matches.push(mem::take(&mut line));
        }
        
        line.clear();
    }
    
    Ok(matches)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(    io::stdin()))),
        _ => Ok(Box::new(BufReader::new(    fs::File::open(filename)?))),
    }
}

fn find_files(paths: &[String], recursive: bool) -> Vec<MyResult<String>> {
    
    let mut results: Vec<MyResult<String>> = Vec::new();
    
    for path in paths {
        match path.as_str() {
            "-" => results.push(Ok(path.to_string())),
            _ => {
                match std::fs::metadata(path) {
                    Err(err) => results.push(Err(From::from(format!("{}: {}", path, err)))),
                    Ok(metadata) => {
                        if metadata.is_file() {
                            results.push(Ok(path.to_string()));
                        } else if metadata.is_dir() {
                            if recursive {
                                for entry_result in WalkDir::new(path) {
                                    if let Ok(entry) = entry_result {
                                        if entry.file_type().is_file() {
                                            results.push(Ok(entry.path().display().to_string()));
                                        }
                                    }
                                }
                            } else {
                                results.push(Err(From::from(format!("{} is a directory", path))));
                            }
                        }
                    },
                }
            },
        }
    }
    
    results
}
