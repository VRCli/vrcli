use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("vrcli").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A simple CLI tool for VRChat API"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("vrcli").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("vrcli"));
}

#[test]
fn test_friends_help() {
    let mut cmd = Command::cargo_bin("vrcli").unwrap();
    cmd.args(&["friends", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Manage friends"));
}

#[test]
fn test_auth_help() {
    let mut cmd = Command::cargo_bin("vrcli").unwrap();
    cmd.args(&["auth", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Configure authentication"));
}

#[test]
fn test_users_help() {
    let mut cmd = Command::cargo_bin("vrcli").unwrap();
    cmd.args(&["users", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("User operations"));
}

#[test]
fn test_worlds_help() {
    let mut cmd = Command::cargo_bin("vrcli").unwrap();
    cmd.args(&["worlds", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("World operations"));
}

// Note: Tests that require actual API authentication should be run separately
// and would require mock servers or test credentials
