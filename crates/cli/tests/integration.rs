//! Integration tests for the idenumbra CLI.

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn cli() -> Command {
    Command::cargo_bin("idenumbra").unwrap()
}

#[test]
fn add_ecu_creates_files_and_manifest() {
    let dir = tempdir().unwrap();
    cli()
        .args(["add", "ECU", "--path", dir.path().to_str().unwrap()])
        .assert()
        .success();
    assert!(dir.path().join("src/idenumbra/countries/ecu/lib.rs").exists());
    assert!(dir.path().join("src/idenumbra/mod.rs").exists());
    assert!(dir.path().join("idenumbra.toml").exists());
    let manifest = std::fs::read_to_string(dir.path().join("idenumbra.toml")).unwrap();
    assert!(manifest.contains("ecu"));
}

#[test]
fn list_shows_installed() {
    let dir = tempdir().unwrap();
    cli()
        .args(["add", "ECU", "ARG", "--path", dir.path().to_str().unwrap()])
        .assert()
        .success();
    cli()
        .args(["list", "--path", dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("ECU"))
        .stdout(predicate::str::contains("ARG"));
}

#[test]
fn remove_deletes_and_updates_manifest() {
    let dir = tempdir().unwrap();
    cli()
        .args(["add", "ECU", "--path", dir.path().to_str().unwrap()])
        .assert()
        .success();
    cli()
        .args(["remove", "ECU", "--path", dir.path().to_str().unwrap()])
        .assert()
        .success();
    assert!(!dir.path().join("src/idenumbra/countries/ecu").exists());
    let manifest = std::fs::read_to_string(dir.path().join("idenumbra.toml")).unwrap();
    assert!(!manifest.contains("ecu"));
}

#[test]
fn dry_run_does_not_write() {
    let dir = tempdir().unwrap();
    cli()
        .args(["add", "ECU", "--path", dir.path().to_str().unwrap(), "--dry-run"])
        .assert()
        .success();
    assert!(!dir.path().join("src/idenumbra/countries/ecu/lib.rs").exists());
    assert!(!dir.path().join("idenumbra.toml").exists());
}

#[test]
fn unknown_country_fails() {
    let dir = tempdir().unwrap();
    cli()
        .args(["add", "ZZZ", "--path", dir.path().to_str().unwrap()])
        .assert()
        .failure();
}

#[test]
fn add_multi() {
    let dir = tempdir().unwrap();
    cli()
        .args(["add", "ECU", "ARG", "MEX", "--path", dir.path().to_str().unwrap()])
        .assert()
        .success();
    for c in ["ecu", "arg", "mex"] {
        assert!(dir
            .path()
            .join(format!("src/idenumbra/countries/{}/lib.rs", c))
            .exists(),
            "missing {}",
            c
        );
    }
}
