use std::{error::Error, fmt::format, num::NonZeroUsize, ops::Range, os::unix::net::UnixListener};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

// ############################################################################

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cutr")
        .version("0.1.0")
        .author("author")
        .about("Rust cut")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true)
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("b")
                .long("bytes")
                .help("Selected bytes")
                .takes_value(true)
                .multiple(false)
                .conflicts_with_all(&["chars", "fields"])
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .short("c")
                .long("chars")
                .help("Selected characters")
                .takes_value(true)
                .multiple(false)
                .conflicts_with_all(&["bytes", "fields"])
        )
        .arg(
            Arg::with_name("delimeter")
                .value_name("DELIMITER")
                .short("d")
                .long("delim")
                .help("Field delimiter")
                .takes_value(true)
                .multiple(false)
                .default_value("\t")
                // .conflicts_with_all(&["bytes", "chars"])
        )
        .arg(
            Arg::with_name("fields")
                .value_name("FIELDS")
                .short("f")
                .long("fields")
                .help("Selected fields")
                .takes_value(true)
                .multiple(false)
                .conflicts_with_all(&["bytes", "chars"])
        )
        .get_matches();
    
    let files = matches.values_of_lossy("files").unwrap();
    println!("files: {:?}", files);
    
    let bytes = matches.value_of("bytes").unwrap();
    println!("bytes: {:?}", bytes);
    
    let chars = matches.value_of("chars").unwrap();
    println!("chars: {:?}", chars);
    
    Ok(Config {
        files: todo!(),
        delimiter: todo!(),
        extract: todo!(),
    })
}

pub fn run(config: Config) -> MyResult<i32> {
    println!("{:?}", config);
    
    Ok(1)
}

// ############################################################################

fn parse_pos(range: &str) -> MyResult<PositionList> {
    unimplemented!();
}

fn parse_index(input: &str) -> Result<usize, String> {
    
    let value_error = || format!("illegal list value: \"{}\"", input);
    
    input
        .starts_with("+").then(|| Err(value_error()))
        .unwrap_or_else(|| {
            // input
            //     .parse()
            Ok("1".to_string())
            // unimplemented!()
        });

    unimplemented!();
}


// ############################################################################

#[test]
fn test_parse_index() {
    let f = |v: &str| {
        v
            .starts_with("+")
            .then(|| Err("starts_with err"))
            .unwrap_or_else(|| {
                v
                    .parse::<NonZeroUsize>()
                    .map(|n| usize::from(n) - 1)
                    .map_err(|_| "parse err")
            })
        };

    let input = "3";
    assert_eq!(f(input), Ok(2));
    
    let input = "+3";
    assert_eq!(f(input), Err("starts_with err"));
    
    let input = "a";
    assert_eq!(f(input), Err("parse err"));
}

#[test]
fn test_bool_then() {
    
    let r = false.then(|| None).map(|b: Option<bool>| "Done");
    assert_eq!(r, None);
    
    let r = true.then(|| None).map(|b: Option<bool>| "Done");
    assert_eq!(r, Some("Done"));
}
