use std::{error::Error, fs, io::{BufRead, BufReader, Read, Seek, SeekFrom}};

use clap::Arg;
use regex::{Regex, RegexBuilder};


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
}

#[derive(Debug)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

// ############################################################################

pub fn get_args() -> MyResult<Config> {
    let matches = clap::App::new("tailr")
        .version("0.1.0")
        .author("masawamor")
        .about("Rust tail")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .takes_value(true)
                .multiple(true)
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .short("n")
                .long("lines")
                .help("line count")
                .takes_value(true)
                .default_value("10")
                .multiple(false)
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .help("byte count")
                .conflicts_with("lines")
                .takes_value(true)
                .multiple(false)
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Suppress headers")
                .required(false)
                .takes_value(false)
                .multiple(false)
        )
        .get_matches();
    
    let files = matches.values_of_lossy("files").unwrap();
    
    let lines = matches.value_of("lines")
        .map(parse_num)
        .transpose()
        .map_err(|err| format!("illegal line count -- {}", err))?;
    
    // let lines = parse_num("val");
    // println!("# lines: {:?}", lines);
    
    let bytes = matches.value_of("bytes")
        .map(parse_num)
        .transpose()
        .map_err(|err| format!("illegal byte count -- {}", err))?;
    
    let quiet = matches.is_present("quiet");
    
    Ok(Config {
        files,
        lines: lines.unwrap(),
        bytes,
        quiet,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("config: {:?}", config);
    
    for filename in config.files {
        match fs::File::open(&filename) {
            Err(err) => eprintln!("{}", err),
            Ok(f) => {
                let r = BufReader::new(f);
                let (num_lines, num_bytes) = count_lines_bytes(&filename)?;
                if let Some(bytes_to_read) = &config.bytes {
                    print_bytes(r, bytes_to_read, num_bytes)?;
                } else {
                    print_lines(r, &config.lines, num_lines)?;
                }
            },
        }
    }
    
    Ok(())
}

// ############################################################################

fn count_lines_bytes(filename: &str) -> MyResult<(i64, i64)> {
    
    let mut file = std::io::BufReader::new(fs::File::open(filename)?);
    
    let mut num_lines = 0;
    let mut num_bytes = 0;
    
    let mut buf = Vec::new();
    loop {
        let bytes_read = file.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            break;
        }
        
        num_lines += 1;
        num_bytes += bytes_read as i64;
        buf.clear();
    }
    
    Ok((num_lines, num_bytes))
}

fn get_start_index(num_to_read: &TakeValue, total: i64) -> Option<u64> {
    
    match num_to_read {
        TakeValue::PlusZero => {
            if total > 0 {
                Some(0)
            } else {
                None
            }            
        },
        TakeValue::TakeNum(num) => {
            if num == &0 || total == 0 || num > &total {
                None
            } else {
                let start = if num < &0 { total + num } else { num - 1 };
                Some(if start < 0 { 0 } else { start as u64 })
            }
        },
    }
}

fn print_lines(mut file: impl BufRead, lines_to_read: &TakeValue, total_lines: i64) -> MyResult<()> {
    
    if let Some(start) = get_start_index(lines_to_read, total_lines) {
        let mut line_num = 0;
        let mut buf = Vec::new();
        loop {
            let bytes_read = file.read_until(b'\n', &mut buf)?;
            if bytes_read == 0 {
                break;
            }
            if line_num >= start {
                print!("{}", String::from_utf8_lossy(&buf));
            }
            line_num += 1;
            buf.clear();
        }
    }
    
    Ok(())
}

fn print_bytes<T>(mut file: T, bytes_to_read: &TakeValue, total_bytes: i64) -> MyResult<()> 
    where T: Read + Seek,
{
    if let Some(start) = get_start_index(bytes_to_read, total_bytes) {
        file.seek(SeekFrom::Start(start))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        if !buf.is_empty() {
            print!("{}", String::from_utf8_lossy(&buf));
        }
    }
    
    Ok(())
}

use once_cell::sync::OnceCell;
static NUM_RE: OnceCell<Regex> = OnceCell::new();

fn parse_num(val: &str) -> MyResult<TakeValue> {
    let num_re = NUM_RE.get_or_init(|| Regex::new(r"^([+-])?(\d+)$").unwrap());
    
    match num_re.captures(val) {
        Some(captures) => {
            let sign = captures.get(1).map_or("-", |m| m.as_str());
            let format_num = format!("{}{}", sign, captures.get(2).unwrap().as_str());
            if let Ok(num) = format_num.parse() {
                if sign == "+" && num == 0 {
                    Ok(TakeValue::PlusZero)
                } else {
                    Ok(TakeValue::TakeNum(num))
                }
            } else {
                Err(From::from(val))
            }
        },
        None => Err(From::from(val)),
    }
}

// ############################################################################

#[test]
fn count_lines_bytes_test() {
    assert_eq!(count_lines_bytes("tests/inputs/count_lines_bytes/0_0.txt").unwrap(), (0, 0));
    assert_eq!(count_lines_bytes("tests/inputs/count_lines_bytes/1_1.txt").unwrap(), (1, 1));
    assert_eq!(count_lines_bytes("tests/inputs/count_lines_bytes/5_25.txt").unwrap(), (5, 25));
}

#[test]
fn get_start_index_test() {
    
    assert_eq!(get_start_index(&TakeValue::PlusZero, 0), None);
    assert_eq!(get_start_index(&TakeValue::PlusZero, 1), Some(0));
    
    assert_eq!(get_start_index(&TakeValue::TakeNum(0), 1), None);
    assert_eq!(get_start_index(&TakeValue::TakeNum(1), 0), None);
    assert_eq!(get_start_index(&TakeValue::TakeNum(2), 1), None);
    
    assert_eq!(get_start_index(&TakeValue::TakeNum(1), 10), Some(0));
    assert_eq!(get_start_index(&TakeValue::TakeNum(2), 10), Some(1));
    assert_eq!(get_start_index(&TakeValue::TakeNum(3), 10), Some(2));
    assert_eq!(get_start_index(&TakeValue::TakeNum(0), 1), None);
    assert_eq!(get_start_index(&TakeValue::TakeNum(1), 0), None);
    assert_eq!(get_start_index(&TakeValue::TakeNum(2), 1), None);
    
    assert_eq!(get_start_index(&TakeValue::TakeNum(-1), 10), Some(9));
    assert_eq!(get_start_index(&TakeValue::TakeNum(-2), 10), Some(8));
    assert_eq!(get_start_index(&TakeValue::TakeNum(-3), 10), Some(7));
    assert_eq!(get_start_index(&TakeValue::TakeNum(-10), 10), Some(0));
    assert_eq!(get_start_index(&TakeValue::TakeNum(-11), 10), Some(0));
}