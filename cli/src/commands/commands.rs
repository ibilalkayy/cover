use clap::{Parser, Subcommand};

use crate::flags::{
    archive::ArchiveData, clean::CleanData, list::ListData, restore::RestoreData,
    schedule::ScheduleData, sync::SyncData,
};

/// Takes the first command that starts the application
///
/// It also contains the author, version and about description of the application
#[derive(Debug, Parser)]
#[clap(
    author = "Bilal Khan",
    version,
    about = "A Rust-based command-line tool for syncing, archiving, restoring, and scheduling backups with ease."
)]
pub struct Cover {
    #[clap(subcommand)]
    pub command: Command,
}

/// Holds the required subcommands for running different features of application
///
/// These subcommands are also pointing to flags that are going to be used
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Keep two folders in sync with each other
    Sync(SyncData),

    /// Create a compressed backup file
    Archive(ArchiveData),

    /// Restore from an archive or backup folder
    Restore(RestoreData),

    /// Setup automatic backups
    Schedule(ScheduleData),

    /// Show history of backups or schedule jobs
    List(ListData),

    /// Remove old backup to save space
    Clean(CleanData),
}
