extern crate tempfile;
extern crate assert_cli;

use tempfile::tempdir;
use std::env;
use std::path::PathBuf;
use std::io::Result;
use std::fs::File;
use std::io::prelude::*;

/// We need the absolute path to the ticket binary under test. We can't use
/// Assert::main_binary or similar because we need to manipulate the current
/// directory that we execute in.const ticket_binary: 
fn ticket_binary() -> Result<PathBuf> {
    Ok(env::current_dir()?
        .join("target")
        .join("debug")
        .join("ticket"))
}

#[test]
fn init_fails_outside_of_git() {
    // There should be no git repo in this tempdir, unless someone has done `git init` in their `/tmp`
    let dir = tempdir().unwrap();

    assert_cli::Assert::command(&[ticket_binary().unwrap()])
        .with_args(&["init"])
        .current_dir(dir.path())
        .fails()
        .and()
        .stderr().is("Failed to install ticket: Can't find a git repository from the current directory.")
        .unwrap();
}

#[test]
fn init_passes_in_git() {
    let dir = tempdir().unwrap();

    // Init a git repo. This means this machine needs a `git` binary on the PATH
    assert_cli::Assert::command(&["git"])
        .with_args(&["init"])
        .current_dir(dir.path())
        .unwrap();

    assert_cli::Assert::command(&[ticket_binary().unwrap()])
        .with_args(&["init"])
        .current_dir(dir.path())
        .stdout().contains("Ticket git hook installed, happy hacking!")
        .unwrap();

    let mut file = File::open(dir.path().join(".git").join("hooks").join("prepare-commit-msg")).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    assert_eq!("#!/bin/sh\nticket insert-ticket-reference $1\n", contents);
}
