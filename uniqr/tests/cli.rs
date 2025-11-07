use assert_cmd::{Command, cargo};
use tempfile::NamedTempFile;

type TestResult = Result<(), Box<dyn std::error::Error>>;

struct Test {
    input: &'static str,
    out: &'static str,
    out_count: &'static str,
}

mod test_case {
    use super::Test;
    
    pub const EMPTY: Test = Test {
        input: "tests/inputs/empty.txt",
        out: "tests/expected/empty.txt.out",
        out_count: "tests/expected/empty.txt.c.out",
    };
    
    pub const ONE: Test = Test {
        input: "tests/inputs/one.txt",
        out: "tests/expected/one.txt.out",
        out_count: "tests/expected/one.txt.c.out",
    };
    
    pub const TWO: Test = Test {
        input: "tests/inputs/two.txt",
        out: "tests/expected/two.txt.out",
        out_count: "tests/expected/two.txt.c.out",
    };
    
    pub const THREE: Test = Test {
        input: "tests/inputs/three.txt",
        out: "tests/expected/three.txt.out",
        out_count: "tests/expected/three.txt.c.out",
    };
    
    pub const SKIP: Test = Test {
        input: "tests/inputs/skip.txt",
        out: "tests/expected/skip.txt.out",
        out_count: "tests/expected/skip.txt.c.out",
    };
    
}

fn run(test: &Test) -> TestResult {
    let expected = std::fs::read_to_string(test.out)?;
    
    Command::new(cargo::cargo_bin!())
        .arg(test.input)
        .assert()
        .success()
        .stdout(expected);
    
    Ok(())
}

fn run_count(test: &Test) -> TestResult {
    let expected = std::fs::read_to_string(test.out_count)?;
    // println!("##### run_count: expected: {}", expected);
    
    Command::new(cargo::cargo_bin!())
        .args(&["-c", test.input])
        .assert()
        .success()
        .stdout(expected);
    
    Ok(())
}

fn run_stdin(test: &Test) -> TestResult {
    let input = std::fs::read_to_string(test.input)?;
    let expected = std::fs::read_to_string(test.out)?;
    
    Command::new(cargo::cargo_bin!())
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    
    Ok(())
}

fn run_stdin_count(test: &Test) -> TestResult {
    let input = std::fs::read_to_string(test.input)?;
    let expected = std::fs::read_to_string(test.out_count)?;
    
    Command::new(cargo::cargo_bin!())
        .arg("-c")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    
    Ok(())
}

fn run_outfile(test: &Test) -> TestResult {
    let expected = std::fs::read_to_string(test.out_count)?;
    
    let outfile = NamedTempFile::new()?;
    let outfile_path = outfile.path().to_str().unwrap();
    
    Command::new(cargo::cargo_bin!())
        .args(&["--count", test.input, outfile_path])
        .assert()
        .success()
        .stdout("");
    
    let contents = std::fs::read_to_string(outfile_path)?;
    
    
    println!("### expected: {}", expected);
    println!("### contants: {}", contents);
   
   
    assert_eq!(&expected, &contents);
    // TODO
    // assert_eq!(expected, contents);
     
    Ok(())
}

// ############################################################################

#[cfg(test)]
mod test {
    use super::*;

    // ############################################################################
    
    #[test]
    fn empty() -> TestResult {
        run(&test_case::EMPTY)
    }
    
    #[test]
    fn empty_count() -> TestResult {
        run_count(&test_case::EMPTY)
    }
    
    #[test]
    fn empty_stdin() -> TestResult {
        run_stdin(&test_case::EMPTY)
    }
    
    #[test]
    fn empty_stdin_count() -> TestResult {
        run_stdin_count(&test_case::EMPTY)
    }
    
    // ############################################################################
    
    #[test]
    fn one() -> TestResult {
        run(&test_case::ONE)
    }
    
    #[test]
    fn one_count() -> TestResult {
        run_count(&test_case::ONE)
    }
    
    #[test]
    fn one_stdin() -> TestResult {
        run_stdin(&test_case::ONE)
    }
    
    #[test]
    fn one_stdin_count() -> TestResult {
        run_stdin_count(&test_case::ONE)
    }
    #[test]
    fn one_output() -> TestResult {
        run_outfile(&test_case::ONE)
    }
    
    // ############################################################################
    
    #[test]
    fn two() -> TestResult {
        run(&test_case::TWO)
    }
    
    #[test]
    fn two_count() -> TestResult {
        run_count(&test_case::TWO)
    }
    
    #[test]
    fn two_stdin() -> TestResult {
        run_stdin(&test_case::TWO)
    }
    
    #[test]
    fn two_stdin_count() -> TestResult {
        run_stdin_count(&test_case::TWO)
    }
    
    // ############################################################################
    
    #[test]
    fn three() -> TestResult {
        run(&test_case::THREE)
    }
    
    #[test]
    fn three_count() -> TestResult {
        run_count(&test_case::THREE)
    }
    
    #[test]
    fn three_stdin() -> TestResult {
        run_stdin(&test_case::THREE)
    }
    
    #[test]
    fn three_stdin_count() -> TestResult {
        run_stdin_count(&test_case::THREE)
    }
    
    // ############################################################################
    
    #[test]
    fn skip() -> TestResult {
        run(&test_case::SKIP)
    }
    
    #[test]
    fn skip_count() -> TestResult {
        run_count(&test_case::SKIP)
    }
    
    #[test]
    fn skip_stdin() -> TestResult {
        run_stdin(&test_case::SKIP)
    }
    
    #[test]
    fn skip_stdin_count() -> TestResult {
        run_stdin_count(&test_case::SKIP)
    }
    
    
}