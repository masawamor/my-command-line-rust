use std::fs;

use assert_cmd::{Command, cargo};
use std::error::Error;
use rand::{Rng, distributions::Alphanumeric};

type TestResult<T> = Result<T, Box<dyn Error>>;

fn gen_bad_filename() -> String {
    
    for _ in 0..100 {
        let filename = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
    
    panic!("gen_bad_filename is failed!");
}

// ############################################################################

#[test]
fn skips_bad_file() -> TestResult<()> {
    let bad = gen_bad_filename();
    let expected = format!("{}: No such file or directory", bad);
    
    Command::new(cargo::cargo_bin!())
        .args(&[&bad, "-b", "1"])
        .assert()
        .success()
        .stderr(predicates::str::contains(expected));
    
    Ok(())
}


