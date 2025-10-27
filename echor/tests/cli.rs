use std::fs;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("USAGE"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    run(&["Hello there"], outfile)
}

#[test]
fn hello1n() -> TestResult {
    let outfile = "tests/expected/hello1.n.txt";
    run(&["-n", "Hello  there"], outfile)
}

#[test]
fn hello2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    run(&["Hello", "there"], outfile)
}

#[test]
fn hello2n() -> TestResult {
    let outfile = "tests/expected/hello2.n.txt";
    run(&["-n", "Hello", "there"], outfile)
}