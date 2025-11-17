use std::{error::Error, fs, io::{self, BufRead, BufReader}, num::NonZeroUsize, ops::Range};

use clap::{App, Arg};
use csv::StringRecord;
use regex::Regex;

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
            Arg::with_name("delimiter")
                .value_name("DELIMITER")
                .short("d")
                .long("delim")
                .help("Field delimiter")
                .takes_value(true)
                .multiple(false)
                .default_value("\t")
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
    
    let delimiter_str = matches.value_of("delimiter").unwrap();
    let delimiter_bytes = delimiter_str.as_bytes();
    let delimiter = if delimiter_bytes.len() != 1 {
        return Err(From::from(format!("--delim \"{}\" must be a single byte", delimiter_str)));
    } else {
        *delimiter_bytes.first().unwrap()
    };
    
    let bytes = matches.value_of("bytes").map(parse_pos).transpose()?;    
    let chars = matches.value_of("chars").map(parse_pos).transpose()?;
    let fields = matches.value_of("fields").map(parse_pos).transpose()?;

    let extract = if let Some(byte_pos) = bytes {
        Extract::Bytes(byte_pos)
    } else if let Some(chars_pos) = chars {
        Extract::Chars(chars_pos)
    } else if let Some(fields_pos) = fields {
        Extract::Fields(fields_pos)
    } else {
        return Err(From::from("Must have --fields, --bytes or --chars"));
    };
    
    Ok(Config {
        files,
        delimiter,
        extract,
    })
}

pub fn run(config: Config) -> MyResult<()> {    
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                match &config.extract {
                    Extract::Fields(ranges) => {
                        let mut rdr = csv::ReaderBuilder::new()
                            .has_headers(false)
                            .delimiter(config.delimiter)
                            .from_path(filename)?;
                        
                        let mut wtr = csv::WriterBuilder::new()
                            .delimiter(config.delimiter)
                            .from_writer(io::stdout());
                    
                        for record in rdr.records() {
                            wtr.write_record(extract_fields(&record?, ranges))?;
                        }
                    },
                    Extract::Bytes(ranges) => {
                        for line in file.lines() {
                            println!("{}", extract_bytes(&line?, ranges));
                        }
                    },
                    Extract::Chars(ranges) => {
                        for line in file.lines() {
                            println!("{}", extract_chars(&line?, ranges));
                        }
                    },
                }
            },
        }
    }
    
    Ok(())
}

// ############################################################################

fn extract_chars(line: &str, position_list: &[Range<usize>]) -> String {
    
    let chars: Vec<_> = line.chars().collect();
    let mut selected: Vec<char> = vec![];
    
    for range in position_list.iter().cloned() {
        for i in range {
            if let Some(c) = chars.get(i) {
                selected.push(*c);
            }
        }
    }
    
    selected.iter().collect()
}

fn extract_bytes(line: &str, position_list: &[Range<usize>]) -> String {
    
    let bytes: Vec<_> = line.bytes().collect();
    let mut selected: Vec<u8> = vec![];
    
    for range in position_list.iter().cloned() {
        for i in range {
            if let Some(b) = bytes.get(i) {
                selected.push(*b);
            }
        }
    }
    
    String::from_utf8_lossy(&selected).into_owned()
}

fn extract_fields(records: &StringRecord, position_list: &[Range<usize>]) -> Vec<String> {
    
    let mut selected: Vec<String> = vec![];
    
    for range in position_list.iter().cloned() {
        for i in range {
            if let Some(val) = records.get(i) {
                selected.push(val.to_string());
            }
        }
    }
    
    selected.iter().map(String::from).collect()
}

#[test]
fn extract_chars_test() {    
    assert_eq!(extract_chars("", &[0..1]), "");
    assert_eq!(extract_chars("aｂcdefd", &[0..1]), "a");
    assert_eq!(extract_chars("aｂcdefd", &[0..1, 3..5]), "ade");
    assert_eq!(extract_chars("aｂcdefd", &[0..1, 3..10]), "adefd");
    assert_eq!(extract_chars("aｂcdefd", &[1..2, 0..4, 4..6]), "ｂaｂcdef");
    assert_eq!(extract_chars("aｂcdefd", &[10..11]), "");
}

#[test]
fn extract_bytes_test() {
    // aあcdefd => 「あ」は3バイト
    assert_eq!(extract_bytes("", &[0..1]), "");
    assert_eq!(extract_bytes("aあcdefd", &[0..1]), "a");
    assert_eq!(extract_bytes("aあcdefd", &[1..2]), "�");
    assert_eq!(extract_bytes("aあcdefd", &[1..4]), "あ");
    assert_eq!(extract_bytes("aあcdefd", &[0..1, 2..6]), "a��cd");
    assert_eq!(extract_bytes("aあcdefd", &[0..1, 0..4, 4..6]), "aaあcd");
    assert_eq!(extract_bytes("aあcdefd", &[10..11]), "");
}

// ############################################################################

fn parse_pos(range: &str) -> MyResult<PositionList> {
    let range_re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    
    range
        .split(",")
        .into_iter()
        .map(|v| {
            parse_index(v)
                .map(|n| { n..n + 1 })
                .or_else(|e| {
                    range_re.captures(v).ok_or(e).and_then(|captures| {
                        let n1 = parse_index(&captures[1])?;
                        let n2 = parse_index(&captures[2])?;
                        if n1 > n2 {
                            return Err(format!("First number({}) in range must be lower than second value({})", n1 + 1, n2 + 1));
                        }
                        Ok(n1..n2 + 1)
                    })
                })
        })
        .collect::<Result<_, _>>()
        .map_err(From::from)
}

fn parse_index(input: &str) -> Result<usize, String> {
    
    let value_error = || format!("illegal list value: \"{}\"", input);
    
    if input.starts_with("+") {
        return Err(value_error());
    }
    
    input
        .parse::<NonZeroUsize>()
        .map(|n| usize::from(n) - 1)
        .map_err(|_| value_error())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(fs::File::open(filename)?))),
    }
}

// ############################################################################

#[test]
fn parse_pos_test() {    
    assert_eq!(parse_pos("1").unwrap(), vec![0..1]);
    assert_eq!(parse_pos("1,2").unwrap(), vec![0..1, 1..2]);
    assert_eq!(parse_pos("1,2,10").unwrap(), vec![0..1, 1..2, 9..10]);
    
    assert_eq!(parse_pos("2-5").unwrap(), vec![1..4]);
    
    assert_eq!(parse_pos("3,6-10").unwrap(), vec![2..3, 5..9]);
    
    assert!(parse_pos("").is_err());
    assert_eq!(parse_pos("").unwrap_err().to_string(), "illegal list value: \"\"".to_string());
    
    assert_eq!(
        parse_pos("5-2").unwrap_err().to_string(), 
        "First number(5) in range must be lower than second value(2)".to_string());
    assert_eq!(
        parse_pos("a-2").unwrap_err().to_string(), 
        "illegal list value: \"a-2\"".to_string());
    assert_eq!(
        parse_pos("5-a").unwrap_err().to_string(), 
        "illegal list value: \"5-a\"".to_string());
}

#[test]
fn parse_index_test() {
    assert_eq!(parse_index("1"), Ok(0));
    assert_eq!(parse_index(""), Err("illegal list value: \"\"".to_string()));
    assert_eq!(parse_index("+2"), Err("illegal list value: \"+2\"".to_string()));
    assert_eq!(parse_index("abc"), Err("illegal list value: \"abc\"".to_string()));
    assert_eq!(parse_index("0"), Err("illegal list value: \"0\"".to_string()));
}

#[test]
fn open_test() {
    assert!(open("src/lib.rs").is_ok());
    assert!(open("-").is_ok());
    assert!(open("kjafliakej").is_err());
}



#[test]
fn split_test() {
    let line = "a b  c d     e f";
    for (i, value) in line.split(" ").enumerate() {
        println!("# {}: {}", i, value);
    }
}

