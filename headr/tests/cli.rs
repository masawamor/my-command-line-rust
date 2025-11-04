// use anyhow::Result;
use assert_cmd::{Command, cargo};
use headr::MyResult;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::fs::{self, File};
use std::io::prelude::*;

// const PRG: &str = "headr";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";
const TWO: &str = "./tests/inputs/two.txt";
const THREE: &str = "./tests/inputs/three.txt";
const TWELVE: &str = "./tests/inputs/twelve.txt";

// --------------------------------------------------
fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename = random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn dies_bad_bytes() -> MyResult<()> {
    let bad = random_string();
    let expected = format!(
        // "invalid value '{bad}' for \
        // '--bytes <BYTES>': invalid digit found in string"
        "illegal byte count -- {bad}"
    );
    
    // Command::cargo_bin(PRG)?
    Command::new(cargo::cargo_bin!())
        .args(["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_lines() -> MyResult<()> {
    let bad = random_string();
    let expected = format!(
        "illegal line count -- {bad}"
    );
    
    Command::new(cargo::cargo_bin!())
        .args(["-n", &bad])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bytes_and_lines() -> MyResult<()> {
    let msg = "The argument '--lines <LINES>' cannot be used with '--bytes <BYTES>'";

    Command::new(cargo::cargo_bin!())
        .args(["-n", "1", "-c", "2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> MyResult<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::new(cargo::cargo_bin!())
        .args([EMPTY, &bad, ONE])
        .assert()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> MyResult<()> {
    // Extra work here due to lossy UTF
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    let output = Command::new(cargo::cargo_bin!()).args(args).output().expect("fail");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);

    Ok(())
}

// --------------------------------------------------
fn run_stdin(
    args: &[&str],
    input_file: &str,
    expected_file: &str,
) -> MyResult<()> {
    // Extra work here due to lossy UTF
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);
    let input = fs::read_to_string(input_file)?;

    let output = Command::new(cargo::cargo_bin!())
        .write_stdin(input)
        .args(args)
        .output()
        .expect("fail");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() ->MyResult<()> {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn empty_n2() ->MyResult<()> {
    run(&[EMPTY, "-n", "2"], "tests/expected/empty.txt.n2.out")
}

// --------------------------------------------------
#[test]
fn empty_n4() ->MyResult<()> {
    run(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")
}

// --------------------------------------------------
#[test]
fn empty_c2() ->MyResult<()> {
    run(&[EMPTY, "-c", "2"], "tests/expected/empty.txt.c2.out")
}

// --------------------------------------------------
#[test]
fn empty_c4() ->MyResult<()> {
    run(&[EMPTY, "-c", "4"], "tests/expected/empty.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn one() -> MyResult<()> {
    run(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn one_n2() ->MyResult<()> {
    run(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4() ->MyResult<()> {
    run(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1() ->MyResult<()> {
    run(&[ONE, "-c", "1"], "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2() ->MyResult<()> {
    run(&[ONE, "-c", "2"], "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4() ->MyResult<()> {
    run(&[ONE, "-c", "4"], "tests/expected/one.txt.c4.out")
}

#[test]
fn one_stdin() -> MyResult<()> {
    run_stdin(&[], ONE, "tests/expected/one.txt.out")
}

#[test]
fn one_n2_stdin() ->MyResult<()> {
    run_stdin(&["-n", "2"], ONE, "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4_stdin() ->MyResult<()> {
    run_stdin(&["-n", "4"], ONE, "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1_stdin() ->MyResult<()> {
    run_stdin(&["-c", "1"], ONE, "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2_stdin() ->MyResult<()> {
    run_stdin(&["-c", "2"], ONE, "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4_stdin() ->MyResult<()> {
    run_stdin(&["-c", "4"], ONE, "tests/expected/one.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn two() -> MyResult<()> {
    run(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn two_n2() ->MyResult<()> {
    run(&[TWO, "-n", "2"], "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4() ->MyResult<()> {
    run(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2() ->MyResult<()> {
    run(&[TWO, "-c", "2"], "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4() ->MyResult<()> {
    run(&[TWO, "-c", "4"], "tests/expected/two.txt.c4.out")
}

#[test]
fn two_stdin() ->MyResult<()> {
    run_stdin(&[], TWO, "tests/expected/two.txt.out")
}

#[test]
fn two_n2_stdin() ->MyResult<()> {
    run_stdin(&["-n", "2"], TWO, "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4_stdin() ->MyResult<()> {
    run_stdin(&["-n", "4"], TWO, "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2_stdin() ->MyResult<()> {
    run_stdin(&["-c", "2"], TWO, "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4_stdin() ->MyResult<()> {
    run_stdin(&["-c", "4"], TWO, "tests/expected/two.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn three() ->MyResult<()> {
    run(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn three_n2() ->MyResult<()> {
    run(&[THREE, "-n", "2"], "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4() ->MyResult<()> {
    run(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2() ->MyResult<()> {
    run(&[THREE, "-c", "2"], "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4() ->MyResult<()> {
    run(&[THREE, "-c", "4"], "tests/expected/three.txt.c4.out")
}

#[test]
fn three_stdin() ->MyResult<()> {
    run_stdin(&[], THREE, "tests/expected/three.txt.out")
}

#[test]
fn three_n2_stdin() ->MyResult<()> {
    run_stdin(&["-n", "2"], THREE, "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4_stdin() ->MyResult<()> {
    run_stdin(&["-n", "4"], THREE, "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2_stdin() ->MyResult<()> {
    run_stdin(&["-c", "2"], THREE, "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4_stdin() ->MyResult<()> {
    run_stdin(&["-c", "4"], THREE, "tests/expected/three.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn twelve() ->MyResult<()> {
    run(&[TWELVE], "tests/expected/twelve.txt.out")
}

#[test]
fn twelve_n2() ->MyResult<()> {
    run(&[TWELVE, "-n", "2"], "tests/expected/twelve.txt.n2.out")
}

#[test]
fn twelve_n4() ->MyResult<()> {
    run(&[TWELVE, "-n", "4"], "tests/expected/twelve.txt.n4.out")
}

#[test]
fn twelve_c2() ->MyResult<()> {
    run(&[TWELVE, "-c", "2"], "tests/expected/twelve.txt.c2.out")
}

#[test]
fn twelve_c4() ->MyResult<()> {
    run(&[TWELVE, "-c", "4"], "tests/expected/twelve.txt.c4.out")
}

#[test]
fn twelve_stdin() ->MyResult<()> {
    run_stdin(&[], TWELVE, "tests/expected/twelve.txt.out")
}

#[test]
fn twelve_n2_stdin() ->MyResult<()> {
    run_stdin(&["-n", "2"], TWELVE, "tests/expected/twelve.txt.n2.out")
}

#[test]
fn twelve_n4_stdin() ->MyResult<()> {
    run_stdin(&["-n", "4"], TWELVE, "tests/expected/twelve.txt.n4.out")
}

#[test]
fn twelve_c2_stdin() ->MyResult<()> {
    run_stdin(&["-c", "2"], TWELVE, "tests/expected/twelve.txt.c2.out")
}

#[test]
fn twelve_c4_stdin() ->MyResult<()> {
    run_stdin(&["-c", "4"], TWELVE, "tests/expected/twelve.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn multiple_files() ->MyResult<()> {
    run(&[EMPTY, ONE, TWO, THREE, TWELVE], "tests/expected/all.out")
}

#[test]
fn multiple_files_n2() ->MyResult<()> {
    run(
        &[EMPTY, ONE, TWO, THREE, TWELVE, "-n", "2"],
        "tests/expected/all.n2.out",
    )
}

#[test]
fn multiple_files_n4() ->MyResult<()> {
    run(
        &["-n", "4", EMPTY, ONE, TWO, THREE, TWELVE],
        "tests/expected/all.n4.out",
    )
}

#[test]
fn multiple_files_c1() ->MyResult<()> {
    run(
        &[EMPTY, ONE, TWO, THREE, TWELVE, "-c", "1"],
        "tests/expected/all.c1.out",
    )
}

#[test]
fn multiple_files_c2() ->MyResult<()> {
    run(
        &[EMPTY, ONE, TWO, THREE, TWELVE, "-c", "2"],
        "tests/expected/all.c2.out",
    )
}

#[test]
fn multiple_files_c4() ->MyResult<()> {
    run(
        &["-c", "4", EMPTY, ONE, TWO, THREE, TWELVE],
        "tests/expected/all.c4.out",
    )
}
