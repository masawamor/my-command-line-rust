use std::error::Error;

use clap::{App, Arg};
use regex::Regex;
use walkdir::WalkDir;

use crate::EntryType::*;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}


// ############################################################################

pub fn run(config: Config) -> MyResult<()> {
    
    // println!("########################");
    // println!("config: {:?}", config);
    // println!("########################");
    
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(err) => eprintln!("{}", err),
                Ok(entry) => {
                    
                    let match_entry_type = if config.entry_types.is_empty() || 
                        config.entry_types.iter().any(|entry_type| {
                            let is_match = match entry_type {
                                Dir => entry.file_type().is_dir(),
                                File => entry.file_type().is_file(),
                                Link => entry.file_type().is_symlink(),
                            };
                            // println!("entry_type is_match: {} = {}", entry.file_name().to_string_lossy(), is_match);
                            is_match
                        }) { true } else { false };
                        
                    let match_re = if config.names.is_empty() ||
                        config.names.iter().any(|re| {
                            let is_match = re.is_match(&entry.file_name().to_string_lossy());
                            // println!("name is_match: {} = {}", entry.file_name().to_string_lossy(), is_match);
                            is_match
                        }) { true } else { false };
                    
                    if match_entry_type && match_re {
                        println!("{}", entry.path().display());
                    }
                    
                },
            }
        }
    }
    
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    
    let matches = App::new("findr")
        .version("0.1.0")
        .author("masawamor")
        .about("Rust find")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .multiple(true)
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .short("n")
                .long("name")
                .help("Name")
                .takes_value(true)
                .multiple(true)
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Entry type")
                .possible_values(&["f", "d", "l"])
                .takes_value(true)
                .multiple(true)
        )
        .get_matches();
    
    let paths = matches.values_of_lossy("paths").unwrap();
    
    let names = matches
        .values_of_lossy("names")
        .map(|vals| {
            vals.into_iter().map(|name| {
                Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name))
            })
            .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();
    
    let entry_types = matches
        .values_of_lossy("types")
        .map(|vals| {
            vals.iter().map(|v|
                match v.as_str() {
                    "f" => File,
                    "d" => Dir,
                    "l" => Link,
                    _ => unreachable!("Invalid type"),
            })
            .collect()
        })
        .unwrap_or_default();
    
    Ok(Config {
        paths,
        names,
        entry_types,
    })
}

#[test]
fn test() -> () {
    // let v1 = vec![Ok(1), Err(9), Ok(3)];
    // let v2 = v1.iter().map(|r| {
    //     r.map_err(|n| { format!("error: {}", n) } )
    // } )
    // .collect::<Result<Vec<i32>, _>>().unwrap();

    // assert_eq!(v2, [1, 9, 3])
}