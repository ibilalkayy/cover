pub mod commands;
pub mod flags;

use crate::commands::commands::{Command, Cover};
use clap::Parser;
use cover_files::{
    archive::archive::ArchiveData, clean::clean::CleanData, list::list::ListData,
    restore::restore::RestoreData, schedule::schedule::ScheduleData, sync::sync::SyncData,
};

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
        Command::Archive(a) => {
            let archive_data = ArchiveData {
                source: a.source,
                zip: a.zip,
                tar: a.tar,
                encrypt: a.encrypt,
                timestamp: a.timestamp,
            };
            archive_data.archive_options();
        }
        Command::Restore(r) => {
            let restore_data = RestoreData {
                overwrite: r.overwrite,
                to: r.to,
                select: r.select,
            };
            restore_data.restore_options();
        }
        Command::Schedule(s) => {
            let schedule_data = ScheduleData {
                daily: s.daily,
                weekly: s.weekly,
                interval: s.interval,
                command: s.command,
            };
            schedule_data.schedule_options();
        }
        Command::List(l) => {
            let list_data = ListData {
                archives: l.archives,
                schedules: l.schedules,
                details: l.details,
            };
            list_data.list_options();
        }
        Command::Clean(c) => {
            let clean_data = CleanData {
                keep_last: c.keep_last,
                older_than: c.older_than,
                dry_run: c.dry_run,
            };
            clean_data.clean_options();
        }
    }
}

/// This is the main function that handles all the commands
fn main() {
    handle_commands();
}
