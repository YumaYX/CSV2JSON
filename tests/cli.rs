use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Read;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "csv2json";

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));

    Ok(())
}

#[test]
fn empty_string() -> TestResult {
    run(
        &["tests/input/empty.csv"],
        "tests/expected/empty_string.txt",
    )
}

#[test]
fn empty_null() -> TestResult {
    run(
        &["tests/input/empty.csv", "-n"],
        "tests/expected/empty_null.txt",
    )
}

#[test]
fn key_value_array() -> TestResult {
    run(
        &["tests/input/key_value.csv"],
        "tests/expected/key_value_array.txt",
    )
}

#[test]
fn key_value_hash() -> TestResult {
    run(
        &["tests/input/key_value.csv", "-k"],
        "tests/expected/key_value_hash.txt",
    )
}
