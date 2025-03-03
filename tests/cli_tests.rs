use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("Groundhog").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Groundhog AI coding assistant"));
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("Groundhog").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Groundhog"));
}

#[test]
fn test_default_run() {
    let mut cmd = Command::cargo_bin("Groundhog").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello, world!"));
}
