#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

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
            println!("Show ticket reference");
        },
        ("set", Some(set_matches)) => {
            println!("Setting ticket reference to {}", set_matches.value_of("TICKET_REFERENCE").unwrap());
        },
        ("insert-ticket-reference", Some(insert_matches)) => {
            println!("Inserting ticket reference to file {}", insert_matches.value_of("COMMIT_MSG_FILE").unwrap());
        },
        ("", None) => println!("A command is required, try `--help`."),
        _ => unreachable!()
    }
}
