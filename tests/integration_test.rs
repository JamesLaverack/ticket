extern crate tempfile;
extern crate assert_cli;

use tempfile::tempdir;
use std::env;
use std::path::PathBuf;
use std::io::Result;

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
