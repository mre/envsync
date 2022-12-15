//! Smoke test for the `envsync` binary
//!
//! Takes the `envsync` binary and runs it against the `fixtures/.env` file.

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envsync")?;
    cmd.arg("fixtures/.env");
    cmd.assert().success().stdout(predicate::str::contains(
        "Creating sample env file: fixtures/.env.sample",
    ));

    // Check contents of the sample file
    let sample_file = std::fs::read_to_string("fixtures/.env.sample")?;

    // Check that it contains the same env vars as the original file
    assert!(sample_file.contains("FOO=<FOO>"));
    assert!(sample_file.contains("BAR=<BAR>"));
    assert!(sample_file.contains("BAZ=<BAZ>"));

    // Remove the sample file
    std::fs::remove_file("fixtures/.env.sample")?;

    Ok(())
}

#[test]
fn smoke_test_replace_example_values() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envsync")?;
    cmd.arg("fixtures/.env");
    cmd.arg("--example");
    cmd.arg("BAR=123");
    cmd.assert().success().stdout(predicate::str::contains(
        "Creating sample env file: fixtures/.env.sample",
    ));

    // Check contents of the sample file
    let sample_file = std::fs::read_to_string("fixtures/.env.sample")?;

    // Check that it contains the same env vars as the original file
    assert!(sample_file.contains("FOO=<FOO>"));
    assert!(sample_file.contains("BAR=123"));
    assert!(sample_file.contains("BAZ=<BAZ>"));

    // Remove the sample file
    std::fs::remove_file("fixtures/.env.sample")?;

    Ok(())
}
