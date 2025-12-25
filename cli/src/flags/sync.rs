use clap::Parser;
use std::path::PathBuf;

/// Configuration options for the `sync` command
///
/// Holds the flags where two of them, source and destination are mandatory
/// and the remaining ones are optional but one is atleast required
///
/// # Example
///
/// ```rust,no_run
/// use my_crate::SyncData;
/// use std::path::PathBuf;
///
/// let sync = SyncData {
///     source: PathBuf::from("source_directory"),
///     destination: PathBuf::from("destination_directory"),
///     changed_only: true,
///     delete: false,
///     dry_run: false,
///     verbose: false,
/// }
/// ```
#[derive(Debug, Parser)]
pub struct SyncData {
    /// Source folder to take the file from
    #[clap(short, long)]
    pub source: PathBuf,

    /// Destination folder to move the file to
    #[clap(short, long)]
    pub destination: PathBuf,

    /// Copy only the changed files
    #[clap(long)]
    pub changed_only: bool,

    /// Remove files in destination not in source
    #[clap(long)]
    pub delete: bool,

    /// Show what would happen after syncing
    #[clap(long)]
    pub dry_run: bool,

    /// Show detailed logs
    #[clap(long)]
    pub verbose: bool,
}
