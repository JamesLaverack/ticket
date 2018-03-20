#[macro_use]
extern crate clap;
extern crate git2;
#[macro_use]
extern crate text_io;

use clap::{Arg, App, SubCommand};
use git2::Repository;
use std::io::{BufRead, BufReader, Read, Write, Error, ErrorKind};
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::env;
use std::io;
use std::fs;

const TICKETFILE_NAME:&'static str = ".ticket";

const GIT_HOOK:&'static str = "#!/bin/sh
ticket insert-ticket-reference $1
";

fn get_repo() -> io::Result<Repository> {
    match Repository::discover(env::current_dir().unwrap()) {
        Ok(repo) => Ok(repo),
        Err(_) => Err(Error::new(ErrorKind::Other, "Can't find a git repository from the current directory.")),
    }
}
    
fn get_ticketfile() -> io::Result<PathBuf> {
    match get_repo()?.workdir() {
            Some(workdir) => Ok(workdir.join(TICKETFILE_NAME)),
            None => Err(Error::new(ErrorKind::Other, "This git repository doesn't have a working directory.")),
        }
}

fn read_commit_msg(filepath:PathBuf) -> io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn update_commit_msg(commit_msg_filepath:PathBuf) -> io::Result<()> {
    let path = commit_msg_filepath.as_path();
    let ticket_reference = read_ticketfile()?;
    let current_msg = read_commit_msg(path.to_path_buf())?;

    let mut final_msg = String::new();
    final_msg.push_str(&ticket_reference);
    // Put in a space for aesthetic reasons
    final_msg.push_str(" ");
    final_msg.push_str(&current_msg);
    
    let mut commit_msg_file = File::create(path)?;
    commit_msg_file.write_all(final_msg.as_bytes())?;

    Ok(())
}

fn read_ticketfile() -> io::Result<String> {
    let mut contents = String::new();
    let ticketfile = get_ticketfile()?;
    return if !ticketfile.exists() {
        Err(Error::new(ErrorKind::Other, "No ticket reference for this repository, use `ticket set` to set one."))
    } else {
        BufReader::new(File::open(ticketfile)?).read_line(&mut contents)?;
        Ok(contents)
    }
}

fn write_ticketfile(ticket_reference :&str) -> io::Result<()> {
    let mut file = File::create(get_ticketfile()?)?;
    file.write_all(ticket_reference.as_bytes())
}

fn confirm(question: &str) -> bool {
    println!("{} (yes/no)", question);
    let line: String = read!("{}\n");
    line.to_lowercase().chars().next().unwrap_or('n') == 'y'
}

fn install_git_hook(force:bool) -> io::Result<()> {
    let repo = get_repo()?;
    let hook_dir = repo.path().join("hooks");
    let hook_path = hook_dir.join("prepare-commit-msg");

    if hook_path.exists() {
        // A prepare-commit-msg hook already exists
        // If we're not told to force overwrite then ask if we should
        if !force {
            // Early return if no permission
            if !confirm("A `prepare-commit-msg` git hook already exists, overwrite?") {
                println!("Bye.");
                return Ok(());
            }
        }

        // Take a backup of the existing hook
        println!("Backing up existing hook...");
        let backup_path = hook_dir.join("prepare-commit-msg.backup");
        fs::rename(hook_path.as_path(), backup_path.as_path())?;
        println!("Existing hook moved to {:?}", backup_path);
    }

    let mut hook_file = OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o770)
        .open(hook_path)?;
    hook_file.write_all(GIT_HOOK.as_bytes())?;
    println!("Ticket git hook installed, happy hacking!");
    Ok(())
}

fn main() {
    let arguments = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("verbose")
             .long("verbose")
             .help("Enable verbose output"))
        .subcommand(SubCommand::with_name("init")
                    .about("Initilise ticket in a git repository by installing the prepare-commit-msg hook.")
                    .arg(Arg::with_name("force")
                         .short("f")
                         .long("force")
                         .help("Force overwriting of any existing hook with no prompt")))
        .subcommand(SubCommand::with_name("remove")
                    .about("Remove ticket from a repository. This removes the prepare-commit-msg hook and removes any .ticket files from the repository root.")
                    .arg(Arg::with_name("force")
                         .short("f")
                         .long("force")
                         .help("Force removal with no prompt")))
        .subcommand(SubCommand::with_name("show")
                    .about("Show the current ticket reference for this repository."))
        .subcommand(SubCommand::with_name("set")
                    .about("Set the ticket reference for this repository.")
                    .arg(Arg::with_name("TICKET_REFERENCE")
                         .help("The new ticket reference")
                         .required(true)
                         .index(1)))
        .subcommand(SubCommand::with_name("insert-ticket-reference")
                    .about("Insert the current ticket reference into a file. You shouldn't need to call this in general operation.")
                    .arg(Arg::with_name("COMMIT_MSG_FILE")
                         .help("Input file for the git commit message")
                         .required(true)
                         .index(1)));

    let matches = arguments.get_matches();

    match matches.subcommand() {
        ("init", Some(init_matches)) => {
            println!("Initilising ticket...");
            match install_git_hook(init_matches.is_present("force")) {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to install ticket: {}", e),
            }
        },
        ("remove", Some(remove_matches)) => {
            if remove_matches.is_present("force") {
                println!("Remove ticket without asking.");
            } else {
                println!("Remove ticket, but ask first.");
            }
        },
        ("show", _) => {
            match read_ticketfile() {
                Ok(ticket_reference) => println!("Ticket reference: {}", ticket_reference),
                Err(error) => eprintln!("{}", error),
            }
        },
        ("set", Some(set_matches)) => {
            let ticket_reference = set_matches.value_of("TICKET_REFERENCE").unwrap();
            match write_ticketfile(ticket_reference) {
                Ok(_) => println!("Set ticket reference to {}", ticket_reference),
                Err(e) => eprintln!("Failed to set ticket reference: {}", e),
            }
        },
        ("insert-ticket-reference", Some(insert_matches)) => {
            match update_commit_msg(PathBuf::from(insert_matches.value_of("COMMIT_MSG_FILE").unwrap())) {
                Ok(_) => {},
                Err(e) => eprintln!("Ticket error: {}", e),
            }
        },
        ("", None) => println!("A command is required, try `--help`."),
        _ => unreachable!()
    }
}
