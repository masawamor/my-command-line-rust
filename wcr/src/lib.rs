use std::{error::Error, io::BufRead};

use clap::Arg;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn run(config: Config) -> MyResult<()> {
    
    let file_num = config.files.len();
    let mut total_info = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };
    
    for filename in &config.files {
        
        match open(filename) {
            Err(err) => eprintln!("{}", err),
            Ok(file) => {
                
                if let Ok(info) = count(file) {
                    show_result(&config, &info, filename);
                    
                    total_info.num_lines += info.num_lines;
                    total_info.num_words += info.num_words;
                    total_info.num_bytes += info.num_bytes;
                    total_info.num_chars += info.num_chars;
                }
            },
        }
    }
    
    if file_num > 1 {
        show_result(&config, &total_info, "total");
    }
    
    Ok(())
}

fn show_result(config: &Config, info: &FileInfo, filename: &str) {
    if config.lines {
        print!("{:>8}", info.num_lines);
    }
    if config.words {
        print!("{:>8}", info.num_words);
    }
    if config.bytes {
        print!("{:>8}", info.num_bytes);
    }
    if config.chars {
        print!("{:>8}", info.num_chars);
    }
    if filename != "-" {
        println!(" {}", filename);
    } else {
        println!();
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    
    let mut line = String::new();
    
    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += line_bytes;
        num_chars += line.chars().count();
        line.clear();
    }   
    
    Ok(FileInfo { num_lines, num_words, num_bytes, num_chars })
}

pub fn get_args() -> MyResult<Config> {
    
    let matches = clap::App::new("wcr")
        .version("0.1.0")
        .author("masawamor")
        .about("Rust wcr")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true)
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Show line count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Show word count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Show byte count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Show character count")
                .takes_value(false)
                .conflicts_with("bytes")
        )
        .get_matches();
    
    let mut param_lines = matches.is_present("lines");
    let mut param_words = matches.is_present("words");
    let mut param_bytes = matches.is_present("bytes");
    let param_chars = matches.is_present("chars"); 
    
    if [param_lines, param_words, param_bytes, param_chars].iter().all(|v| v == &false) {
        param_lines = true;
        param_words = true;
        param_bytes = true;
    }
    
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: param_lines,
        words: param_words,
        bytes: param_bytes,
        chars: param_chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(std::io::BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(std::io::BufReader::new(std::fs::File::open(filename)?))),
    }
}

#[cfg(test)]
mod tests {
    use super::{FileInfo, count};
    use std::io::Cursor;
    
    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info_result = count(Cursor::new(text));
        assert!(info_result.is_ok());
        
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        
        assert_eq!(info_result.unwrap(), expected);
    }
}