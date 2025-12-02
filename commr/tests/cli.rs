use std::error::Error;

use assert_cmd::{Command, cargo, output::OutputOkExt};
use predicates::prelude::*;

type TestResult<T> = Result<T, Box<dyn Error>>;

#[test]
fn get_args_test_no_args() -> TestResult<()> {
    
    let output = Command::new(cargo::cargo_bin!())
        .assert()
        .failure()
        .get_output().clone();
    
    let stderr_str = String::from_utf8_lossy(&&output.stderr).to_string();

    let pred_error_msg= predicates::str::contains("error: The following required arguments were not provided:");
    assert!(pred_error_msg.eval(&stderr_str));
    
    let pred_usage= predicates::str::contains("USAGE:");
    assert!(pred_usage.eval(&stderr_str));    
    
    Ok(())
}

#[test]
fn run_test_both_input_files_cannot_be_stdin() -> TestResult<()> {
    
    Command::new(cargo::cargo_bin!())
        .args(&["-", "-"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Both input files cannot be STDIN (\"-\")"));
    
    Ok(())
}