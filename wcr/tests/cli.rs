use assert_cmd::{Command, cargo};

use wcr::MyResult;

type TestResult<T> = MyResult<T>;

const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const ATLAMAL: &str = "tests/inputs/atlamal.txt";

fn run(args: &[&str], expected_file: &str) -> TestResult<()> {
    let expected = std::fs::read_to_string(expected_file)?;
    
    Command::new(cargo::cargo_bin!())
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    
    Ok(())
}

fn run_stdin(args: &[&str], stdin_file: &str, expected_file: &str) -> TestResult<()> {
    
    let stdin_str = std::fs::read_to_string(stdin_file)?;
    let expected = std::fs::read_to_string(expected_file)?;
    
    Command::new(cargo::cargo_bin!())
        .args(args)
        .write_stdin(stdin_str)
        .assert()
        .success()
        .stdout(expected);
    
    Ok(())
}

// #[test]
// fn test_get_args_no_flags() {
//     Command::new(cargo::cargo_bin!())
//         .assert()
//         .success()
//         .stdout(predicates::str::contains("Config { files: [\"-\"], lines: true, words: true, bytes: true, chars: true }"));
// }


#[test]
fn empty_bytes() -> TestResult<()>{
    run(&["--bytes", EMPTY], "tests/expected/empty.txt.c.out")
}

#[test]
fn empty_lines_bytes() -> TestResult<()>{
    run(&["--lines", "--bytes", EMPTY], "tests/expected/empty.txt.cl.out")
}

#[test]
fn empty_lines() -> TestResult<()>{
    run(&["--lines", EMPTY], "tests/expected/empty.txt.l.out")
}

#[test]
fn empty_lines_words_chars() -> TestResult<()>{
    run(&["--lines", "--words", "--chars", EMPTY], "tests/expected/empty.txt.lwm.out")
}

#[test]
fn empty_chars() -> TestResult<()>{
    run(&["--chars", EMPTY], "tests/expected/empty.txt.m.out")
}

#[test]
fn empty_lines_chars() -> TestResult<()> {
    run(&["--lines", "--chars", EMPTY], "tests/expected/empty.txt.ml.out")
}

#[test]
fn empty() -> TestResult<()>{
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_words() -> TestResult<()>{
    run(&["--words", EMPTY], "tests/expected/empty.txt.w.out")
}

#[test]
fn empty_words_bytes() -> TestResult<()> {
    run(&["--words", "--bytes", EMPTY], "tests/expected/empty.txt.wc.out")
}

#[test]
fn empty_lines_words_() -> TestResult<()> {
    run(&["--lines", "--words", EMPTY], "tests/expected/empty.txt.wl.out")
}

#[test]
fn empty_words_chars() -> TestResult<()> {
    run(&["--words", "--chars", EMPTY], "tests/expected/empty.txt.wm.out")
}

// ##################################################################################

#[test]
fn fox_bytes() -> TestResult<()>{
    run(&["--bytes", FOX], "tests/expected/fox.txt.c.out")
}

#[test]
fn fox_lines_bytes() -> TestResult<()>{
    run(&["--lines", "--bytes", FOX], "tests/expected/fox.txt.cl.out")
}

#[test]
fn fox_lines() -> TestResult<()>{
    run(&["--lines", FOX], "tests/expected/fox.txt.l.out")
}

#[test]
fn fox_lines_words_chars() -> TestResult<()>{
    run(&["--lines", "--words", "--chars", FOX], "tests/expected/fox.txt.lwm.out")
}

#[test]
fn fox_chars() -> TestResult<()>{
    run(&["--chars", FOX], "tests/expected/fox.txt.m.out")
}

#[test]
fn fox_lines_chars() -> TestResult<()> {
    run(&["--lines", "--chars", FOX], "tests/expected/fox.txt.ml.out")
}

#[test]
fn fox() -> TestResult<()>{
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_words() -> TestResult<()>{
    run(&["--words", FOX], "tests/expected/fox.txt.w.out")
}

#[test]
fn fox_words_bytes() -> TestResult<()> {
    run(&["--words", "--bytes", FOX], "tests/expected/fox.txt.wc.out")
}

#[test]
fn fox_lines_words_() -> TestResult<()> {
    run(&["--lines", "--words", FOX], "tests/expected/fox.txt.wl.out")
}

#[test]
fn fox_words_chars() -> TestResult<()> {
    run(&["--words", "--chars", FOX], "tests/expected/fox.txt.wm.out")
}

// ##################################################################################

#[test]
fn atlamal_bytes() -> TestResult<()>{
    run(&["--bytes", ATLAMAL], "tests/expected/atlamal.txt.c.out")
}

#[test]
fn atlamal_lines_bytes() -> TestResult<()>{
    run(&["--lines", "--bytes", ATLAMAL], "tests/expected/atlamal.txt.cl.out")
}

#[test]
fn atlamal_lines() -> TestResult<()>{
    run(&["--lines", ATLAMAL], "tests/expected/atlamal.txt.l.out")
}

#[test]
fn atlamal_lines_words_chars() -> TestResult<()>{
    run(&["--lines", "--words", "--chars", ATLAMAL], "tests/expected/atlamal.txt.lwm.out")
}

#[test]
fn atlamal_chars() -> TestResult<()>{
    run(&["--chars", ATLAMAL], "tests/expected/atlamal.txt.m.out")
}

#[test]
fn atlamal_lines_chars() -> TestResult<()> {
    run(&["--lines", "--chars", ATLAMAL], "tests/expected/atlamal.txt.ml.out")
}

#[test]
fn atlamal() -> TestResult<()>{
    run(&[ATLAMAL], "tests/expected/atlamal.txt.out")
}

#[test]
fn atlamal_words() -> TestResult<()>{
    run(&["--words", ATLAMAL], "tests/expected/atlamal.txt.w.out")
}

#[test]
fn atlamal_words_bytes() -> TestResult<()> {
    run(&["--words", "--bytes", ATLAMAL], "tests/expected/atlamal.txt.wc.out")
}

#[test]
fn atlamal_lines_words_() -> TestResult<()> {
    run(&["--lines", "--words", ATLAMAL], "tests/expected/atlamal.txt.wl.out")
}

#[test]
fn atlamal_words_chars() -> TestResult<()> {
    run(&["--words", "--chars", ATLAMAL], "tests/expected/atlamal.txt.wm.out")
}

#[test]
fn atlamal_stdin() -> TestResult<()> {
    run_stdin(&[], ATLAMAL, "tests/expected/atlamal.txt.stdin.out")
}

// ##################################################################################

#[test]
fn all_bytes() -> TestResult<()>{
    run(&["--bytes", EMPTY, FOX, ATLAMAL], "tests/expected/all.c.out")
}

#[test]
fn all_lines_bytes() -> TestResult<()>{
    run(&["--lines", "--bytes", EMPTY, FOX, ATLAMAL], "tests/expected/all.cl.out")
}

#[test]
fn all_lines() -> TestResult<()>{
    run(&["--lines", EMPTY, FOX, ATLAMAL], "tests/expected/all.l.out")
}

#[test]
fn all_lines_words_chars() -> TestResult<()>{
    run(&["--lines", "--words", "--chars", EMPTY, FOX, ATLAMAL], "tests/expected/all.lwm.out")
}

#[test]
fn all_chars() -> TestResult<()>{
    run(&["--chars", EMPTY, FOX, ATLAMAL], "tests/expected/all.m.out")
}

#[test]
fn all_lines_chars() -> TestResult<()> {
    run(&["--lines", "--chars", EMPTY, FOX, ATLAMAL], "tests/expected/all.ml.out")
}

#[test]
fn all() -> TestResult<()>{
    run(&[EMPTY, FOX, ATLAMAL], "tests/expected/all.out")
}

#[test]
fn all_words() -> TestResult<()>{
    run(&["--words", EMPTY, FOX, ATLAMAL], "tests/expected/all.w.out")
}

#[test]
fn all_words_bytes() -> TestResult<()> {
    run(&["--words", "--bytes", EMPTY, FOX, ATLAMAL], "tests/expected/all.wc.out")
}

#[test]
fn all_lines_words_() -> TestResult<()> {
    run(&["--lines", "--words", EMPTY, FOX, ATLAMAL], "tests/expected/all.wl.out")
}

#[test]
fn all_words_chars() -> TestResult<()> {
    run(&["--words", "--chars", EMPTY, FOX, ATLAMAL], "tests/expected/all.wm.out")
}
