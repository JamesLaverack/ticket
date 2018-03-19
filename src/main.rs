#[macro_use]
extern crate clap;
extern crate git2;

use clap::{Arg, App, SubCommand};
use git2::Repository;
use std::io::{BufRead, BufReader, Read, Write, Error, ErrorKind};
use std::fs::File;
use std::path::PathBuf;
use std::env;
use std::io;

const TICKETFILE_NAME:&'static str = ".ticket";

fn get_ticketfile() -> io::Result<PathBuf> {
    match Repository::discover(env::current_dir().unwrap()) {
        Ok(repo) => match repo.workdir() {
            Some(workdir) => {
                return Ok(workdir.join(TICKETFILE_NAME));

            },
            None => return Err(Error::new(ErrorKind::Other, "This git repository doesn't have a working directory.")),
        }
        Err(_) => return Err(Error::new(ErrorKind::Other, "Can't find a git repository from the current directory.")),
    };
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

fn main() {
    let arguments = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("verbose")
             .long("verbose")
             .help("Enable verbose output"))
        .subcommand(SubCommand::with_name("init")
                    .about("Initilise ticket in a git repository by installing the prepare-commit-msg hook."))
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
        ("init", _) => {
            println!("Init ticket");
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
