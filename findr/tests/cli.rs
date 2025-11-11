use std::{error::Error, fs};

use assert_cmd::{Command, cargo};
use rand::{Rng, distributions::Alphanumeric};

type TestResult = Result<(), Box<dyn Error>>;

// ############################################################################

fn gen_bad_filename() -> String {
    loop {
        let filename = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let contents = fs::read_to_string(expected_file)?;
    let mut expected: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    expected.sort();
    
    let cmd = Command::new(cargo::cargo_bin!())
        .args(args)
        .assert()
        .success();
    
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let mut lines: Vec<&str> = stdout.split("\n").filter(|s| !s.is_empty()).collect();
    lines.sort();
    
    assert_eq!(lines, expected);
    
    Ok(())
}

// ############################################################################

#[test]
fn dies_bad_type() -> TestResult {
    Command::new(cargo::cargo_bin!())
        .args(&["--type", "x"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("error: 'x' isn't a valid value for '--type <TYPE>...'"));
        
    Ok(())
}

#[test]
fn dies_bad_name() -> TestResult {
    Command::new(cargo::cargo_bin!())
        .args(&["--name", "*.csv"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Invalid --name \"*.csv\""));
        
    Ok(())
}

#[test]
fn skips_bad_dir() -> TestResult {
    let bad_file = gen_bad_filename();
    let expected = format!("IO error for operation on {}: No such file or directory (os error 2)", bad_file);
    
    Command::new(cargo::cargo_bin!())
        .args(&[&bad_file])
        .assert()
        .success()
        .stderr(predicates::str::contains(expected));
        
    Ok(())
}

// ############################################################################


#[test]
fn name_a() -> TestResult {
    run(&["tests/inputs/a", "-n", "a"], "tests/expected/name_a.txt")
}

#[test]
fn path_a() -> TestResult {
    run(&["tests/inputs/a"], "tests/expected/path_a.txt")
}
