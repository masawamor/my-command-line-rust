use std::error::Error;
use chrono::{Datelike, Local, NaiveDate};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

// ############################################################################

pub fn get_args() -> MyResult<Config> {
    let matches = clap::App::new("calr")
        .version("0.1.0")
        .author("masawamor")
        .about("Rust calr")
        .arg(
            clap::Arg::with_name("month")
        )
        .get_matches();
    
    let today = Local::now();
    
    Ok(Config {
            month: Some(today.month()),
            year: 1,
            today: today.date_naive()
        }
    )
}

pub fn run(config: Config) -> MyResult<()> {
    println!("config: {:?}", config);
    Ok(())
}
