pub mod commands;
pub mod flags;

use crate::commands::commands::{Command, Cover};
use clap::Parser;
use cover_files::sync::sync::SyncData;

/// This function handles all the commands of the Cover application.
///
/// The commands are: sync, archive, restore, schedule, list, and clean.
///
/// # Example
///
/// ```rust,no_run
/// // Example of calling the function in main.rs
/// use my_crate::handle_commands;
///
/// fn main() {
///     handle_commands();
/// }
/// ```
fn handle_commands() {
    let cover = Cover::parse();
    match cover.command {
        Command::Sync(s) => {
            let mut sync_data = SyncData {
                source: s.source,
                destination: s.destination,
                changed_only: s.changed_only,
                delete: s.delete,
                dry_run: s.dry_run,
                verbose: s.verbose,
            };
            sync_data.sync_options();
        }
    }
}

/// This is the main function that handles all the commands
fn main() {
    handle_commands();
}
