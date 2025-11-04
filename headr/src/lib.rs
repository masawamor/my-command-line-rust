use std::{error::Error, io::{BufRead, BufReader, Read}};

use clap::Arg;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    let file_counts = config.files.len();
    
    for (file_no, file) in config.files.iter().enumerate() {
        match open(&file) {
            Err(err) => eprintln!("{}: {}", file, err),
            Ok(mut reader) => {
                if file_counts > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_no > 0 { "\n" } else { "" },
                        file
                    );
                }
                
                if let Some(bytes_to_read) = config.bytes {
                    let mut handle = reader.take(bytes_to_read as u64);
                    let mut buffer = vec![0; bytes_to_read];
                    let bytes = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes]));
                    
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = reader.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            },
        }
    }
    
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = clap::App::new("headr")
        .version("0.1.0")
        .author("author")
        .about("Rust head")
        .arg(
            Arg::with_name("lines")
                .long("lines")
                .short("n")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
                .long("bytes")
                .short("c")
                .value_name("BYTES")
                .help("Number of bytes")
                .conflicts_with("lines")
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )
        .get_matches();
    
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;
    
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;
    
    Ok(
        Config {
            files: matches.values_of_lossy("files").unwrap(),
            lines: lines.unwrap(),
            bytes,
        }
    )
}


fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) => {
            if n > 0 {
                // println!("Ok");
                Ok(n)
            } else {
                // println!("Ok else Err");
                Err(From::from(val))
            }
        },
        _ => {
            // println!("Not Ok Err");
            Err(From::from(val))
        }
    }
}

#[test]
fn test_parse_positive_int() {
   assert_eq!(1, parse_positive_int("1").unwrap());
   assert_eq!(10, parse_positive_int("10").unwrap());
   assert!(parse_positive_int("0").is_err());
   assert!(parse_positive_int("-1").is_err());
   assert!(parse_positive_int("a").is_err());
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(filename)?))),
    }
}
