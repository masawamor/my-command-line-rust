
use std::{error::Error, fs, io::{self, BufRead, BufReader}};

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

enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
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
    let file1 = &config.file1;
    let file2 = &config.file2;
    
    if file1 == "-" && file2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }
    
    let case = |line: String| {
        if config.insensitive {
            line.to_uppercase()
        } else {
            line
        }
    };
    
    let mut file1_iter = open(&file1)?.lines().filter_map(Result::ok).map(case);
    let mut line1 = file1_iter.next();
    
    let mut file2_iter = open(&file2)?.lines().filter_map(Result::ok).map(case);
    let mut line2 = file2_iter.next();
    
    let print = |col: Column| {
        let mut columns = vec![];
        match col {
            Column::Col1(val) => {
                if config.show_col1 {
                    columns.push(val);
                }
            },
            Column::Col2(val) => {
                if config.show_col2 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            },
            Column::Col3(val) => {
                if config.show_col3 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    if config.show_col2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            },
        }
        if !columns.is_empty() {
            println!("{}", columns.join(&config.delimiter));
        }
    };
    
    // println!("# before: {:?}, {:?}", line1, line2);
    
    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(str1), Some(str2)) => {
                // println!("# 3: {}, {}", str1, str2);
                match str1.cmp(str2) {
                    std::cmp::Ordering::Equal => {
                        print(Column::Col3(str1));
                        line1 = file1_iter.next();
                        line2 = file2_iter.next();
                    },
                    std::cmp::Ordering::Less => {
                        print(Column::Col1(str1));
                        line1 = file1_iter.next();
                    },
                    std::cmp::Ordering::Greater => {
                        print(Column::Col2(str2));
                        line2 = file2_iter.next();
                    },
                }
            },
            (Some(str1), None) => {
                // println!("# 1: {}", str1);
                print(Column::Col1(str1));
                line1 = file1_iter.next();
            },
            (None, Some(str2)) => {
                // println!("# 2: {}", str2);
                print(Column::Col2(str2));
                line2 = file2_iter.next();
            },
            _ => {
                // println!("str1: {}, str2: {}", "None", "None");
                ()
            },
        }
    }
    

    
    
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
                fs::File::open(filename).map_err(|e| format!("{}: {}", filename, e))?
            ))),
    }
}