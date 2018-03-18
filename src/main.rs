extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("ticket")
        .version("0.1.0")
        .author("James Laverack <james@jameslaverack.com>")
        .about("Automatically insert ticket references into git commit messages.")
        .arg(Arg::with_name("verbose")
             .long("verbose")
             .help("Enable verbose output"))
        .subcommand(SubCommand::with_name("init")
                    .about("Initilise ticket in a git repository by installing the prepare-commit-msg hook."))
        .subcommand(SubCommand::with_name("uninstall")
                    .about("Remove ticket from a repository. This removes the prepare-commit-msg hook and removes any .ticket files from the repository root."))
        .subcommand(SubCommand::with_name("show")
                    .about("Show the current ticket reference for this repository."))
        .subcommand(SubCommand::with_name("set")
                    .about("Set the ticket reference for this repository."))
        .subcommand(SubCommand::with_name("insert-ticket")
                    .about("Insert the current ticket reference into a file. You shouldn't need to call this in general operation.")
                    .arg(Arg::with_name("COMMIT_MSG")
                         .help("Input file for the git commit message")
                         .required(true)
                         .index(1)))
        .get_matches();

    println!("Hello, git.");
}
