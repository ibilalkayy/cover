pub mod commands;
pub mod flags;

use crate::commands::commands::{Command, Cover};
use clap::Parser;

fn handle_commands() {
    let cover = Cover::parse();
    match cover.command {
        Command::Sync(_s) => println!("Sync data"),
        Command::Archive(_a) => println!("Archive data"),
        Command::Restore(_r) => println!("Restore data"),
        Command::Schedule(_s) => println!("Schedule data"),
        Command::List(_l) => println!("List data"),
        Command::Clean(_c) => println!("Clean data"),
    }
}

fn main() {
    handle_commands();
}
